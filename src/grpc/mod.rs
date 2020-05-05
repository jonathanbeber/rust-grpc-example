use futures::Future;
use slog::Logger;

use super::{proto, Stock};

/// `client` hold the logic for the client generate gRPC requests.
pub mod client;

/// `Handler` is the gRPC interface responsible for receiving requests and
///  executing a [`Stock`](../trait.Stock.html) implementation.
///
/// The main motivation for this struct is to separate gRPC server logic
/// and initialization from its own tests.
#[derive(Clone)]
pub struct Handler<S>
where
    S: Stock + Send + Clone + 'static,
{
    stock: S,
    log: Logger,
}

impl<S> Handler<S>
where
    S: Stock + Send + Clone + 'static,
{
    /// Returns a instance of `Handler` given a
    /// [`Stock`](../trait.Stock.html) implementation and a
    /// [slog::Logger](https://docs.rs/slog/2.5.2/slog/struct.Logger.html)
    /// .
    pub fn new(stock: S, log: Logger) -> Self {
        Self { stock, log }
    }

    fn handle(&self, req: proto::StockRequest) -> proto::StockResponse {
        trace!(self.log, "Starting to handle request");
        let mut resp = proto::StockResponse::new();
        resp.set_items(protobuf::RepeatedField::from_vec(
            self.stock
                .get(req.get_store(), req.get_display_unavailable_items()),
        ));
        trace!(self.log, "Finishing request");
        resp
    }
}

/// Service implements the
/// [`crate::proto::Stock`](../proto/stock_grpc/trait.Stock.html) trait
/// reponsible for handling gRPC requests.
///
/// The main motivation for this struct is to separate gRPC server logic
/// and initialization from the [`Handler`'s](struct.Handler.html) tests.
#[derive(Clone)]
pub struct Service<S>
where
    S: Stock + Send + Clone + 'static,
{
    handler: Handler<S>,
    log: Logger,
}

impl<S> proto::StockService for Service<S>
where
    S: Stock + Send + Clone + 'static,
{
    fn list_items(
        &mut self,
        ctx: grpcio::RpcContext,
        req: proto::StockRequest,
        sink: grpcio::UnarySink<proto::StockResponse>,
    ) {
        trace!(self.log, "gRPC request received");
        let resp = self.handler.handle(req);

        let log = Logger::new(&self.log, o!("component" => "gRPCSink"));
        let f = sink
            .success(resp)
            .map_err(move |err| error!(log, "Error while replying StockRequest: {:?}", err));
        ctx.spawn(f);
        trace!(self.log, "Finished gRPC request");
    }
}

impl<S> Service<S>
where
    S: Stock + Send + Clone + 'static,
{
    /// Returns a instance of `Service` given a
    /// [`Handler`](struct.Handler.html) and a
    /// [slog::Logger](https://docs.rs/slog/2.5.2/slog/struct.Logger.html)
    /// .
    pub fn new(handler: Handler<S>, log: Logger) -> Self {
        Self { handler, log }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[derive(Clone)]
    struct StockMock {
        resp: Vec<proto::Item>,
    }

    impl Stock for StockMock {
        fn get(&self, _: proto::Store, _: bool) -> Vec<proto::Item> {
            self.resp.clone()
        }
    }

    #[test]
    fn test_handler() {
        let mut item = proto::Item::new();
        item.name = String::from("Laptop UltraPower");
        item.brand = String::from("GNUFoundation");
        item.category = String::from("computers/desktop-computers");
        item.quantity = 4;
        item.availability_description = proto::AvailabilityDescription::AVAILABLE;

        let mut item2 = proto::Item::new();
        item2.name = String::from("Laptop air");
        item2.brand = String::from("AbCorp");
        item2.category = String::from("computers/laptops");
        item2.quantity = 8;
        item.availability_description = proto::AvailabilityDescription::AVAILABLE;

        let expected_items = vec![item, item2];

        let log = crate::tests::log::new();
        let handler = grpc::Handler::new(
            StockMock {
                resp: expected_items.clone(),
            },
            log,
        );

        let mut request = proto::StockRequest::new();
        request.set_store(proto::Store::BERLIN_DE);

        let mut response = proto::StockResponse::new();
        response.set_items(protobuf::RepeatedField::from_vec(expected_items));

        assert_eq!(handler.handle(request), response);
    }
}
