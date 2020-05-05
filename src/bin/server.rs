use std::net::SocketAddr;
use std::sync::{atomic, Arc};

#[macro_use]
extern crate slog;

use futures::Future;
use slog::Drain;
use structopt::StructOpt;

use stock::{grpc, proto, Product, StockImpl};

/// Opt is the struct responsible for holding the server initialization options.
#[derive(Debug, StructOpt)]
#[structopt(
    name = format!("{}-{}", env!("CARGO_PKG_NAME"), "server"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
struct Opt {
    /// Sets the listen address.
    #[structopt(long, default_value = "127.0.0.1:9090")]
    addr: SocketAddr,
}

fn main() {
    // Parses cli options
    let opt = Opt::from_args();

    // Sets up logs
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = slog::Logger::root(
        drain,
        o!(
            "version" => Opt::clap().p.meta.version.unwrap_or("0.0.1"),
            "address" => opt.addr
        ),
    );

    // Sets up gracefull shutdown
    let running = Arc::new(atomic::AtomicBool::new(true));
    let r = running.clone();
    if let Err(e) = ctrlc::set_handler(move || {
        r.store(false, atomic::Ordering::SeqCst);
    }) {
        error!(log, "Could not set signal handler: {:?}", e);
        std::process::exit(2);
    }

    // Generates fake data to the server as it does not have a persistent
    // storage yet.
    trace!(log, "Creating fake data for the server");
    let mut stock = StockImpl::new(log.new(o!("component" => "stock")));
    stock.register(
        Product::new("Laptop air", "AbCorp", "computers/laptops"),
        proto::Store::BERLIN_DE,
        8,
    );
    stock.register(
        Product::new(
            "Laptop UltraPower",
            "GNUFoundation",
            "computers/desktop-computers",
        ),
        proto::Store::BERLIN_DE,
        4,
    );
    stock.register(
        Product::new("Model 3", "Tesla", "cars"),
        proto::Store::BERLIN_DE,
        0,
    );
    stock.register(
        Product::new(
            "SantosDumond watch Active2",
            "EvilCorp",
            "wearables/smartwatches",
        ),
        proto::Store::VENEZA_IT,
        3,
    );
    stock.register(
        Product::new("Model 3", "Tesla", "cars"),
        proto::Store::VENEZA_IT,
        0,
    );

    // Geerates and starts gRPC server instance
    trace!(log, "Creating server");
    let handler = grpc::Handler::new(stock, log.new(o!("component" => "handler")));
    let stock_service = grpc::Service::new(handler, log.new(o!("component" => "service")));

    let env = Arc::new(grpcio::Environment::new(1));
    let service = proto::create_stock_service(stock_service);
    let mut server = match grpcio::ServerBuilder::new(env)
        .register_service(service)
        .bind(opt.addr.ip().to_string(), opt.addr.port())
        .build()
    {
        Ok(server) => server,
        Err(e) => {
            error!(log, "Failed to initialize server: {:?}", e);
            std::process::exit(2);
        }
    };

    server.start();
    for &(ref host, port) in server.bind_addrs() {
        info!(log, "listening on {}:{}", host, port);
    }

    // Handles signals for graceful shutdown
    while running.load(atomic::Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    info!(log, "Got signal! Exiting...");
    let _ = server.shutdown().wait();
}
