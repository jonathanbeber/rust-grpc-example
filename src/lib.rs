#![deny(missing_docs)]
//! `stock` represents a stock service. It holds information about
//! different stores and returns them when requested.
//!
//! Currently, it uses in-memory ephemeral storage, based on rust
//! collections. Also, it implements a gRPC interface for serving client's
//! requests.

use std::{cmp, collections, hash};

#[macro_use]
extern crate slog;

use slog::Logger;

/// `grpc` holds all the gRPC logic used by the stock service.
pub mod grpc;
/// `proto` is generated during the build. It's the result of the
/// build script.
pub mod proto;

/// `Stock` is the main representation of the crate. This trait is
/// responsible for holding all the business methods needed while allowing
/// external interfaces handlers (HTTP, gRPC, etc.) tests without
/// attaching its own logic.
/// `grpc.tests.StockMock` is an example of the motivation for this trait.
///
///  # Examples
///
/// ```
/// # use stock::{proto, Stock};
/// # fn main() {
/// struct StockMock;
///
/// impl Stock for StockMock {
///     fn get(&self, _: proto::Store, _: bool) -> Vec<proto::Item> {
///         vec![proto::Item::new()]
///     }
/// }
/// # }
/// ```
pub trait Stock {
    /// Returns a `Vec` of [`Item`s](proto/stock/struct.Item.html) given a
    /// [Store](proto/stock/enum.Store.html). Also, it receives a boolean
    /// parameter `show_unavailable` that will decide if unavailable items
    /// must be returned.
    fn get(&self, store: proto::Store, show_unavaible: bool) -> Vec<proto::Item>;
}

/// `StockImpl` is the default implementation for
/// [`Stock`](trait.Stock.html). It holds a in-memory ephemeral
/// storage based on rust collection `HashMap`.
#[derive(Clone)]
pub struct StockImpl {
    stores: collections::HashMap<proto::Store, collections::HashMap<Product, u32>>,
    log: Logger,
}

impl Stock for StockImpl {
    fn get(&self, store: proto::Store, show_unavaible: bool) -> Vec<proto::Item> {
        trace!(self.log, "Starting get operation on store {:?}", &store);
        let mut answ = vec![];
        if let Some(products) = self.stores.get(&store) {
            for (product, qt) in products {
                if *qt <= 0 && !show_unavaible {
                    continue;
                }
                let mut item = proto::Item::new();
                item.name = product.name.clone();
                item.brand = product.brand.clone();
                item.category = product.category.clone();
                item.quantity = *qt as i32;
                item.availability_description = match *qt {
                    x if x <= 0 => proto::AvailabilityDescription::UNAVAILABLE,
                    _ => proto::AvailabilityDescription::AVAILABLE,
                };
                trace!(self.log, "Found item {:?}", &item);
                answ.push(item);
            }
        }
        debug!(
            self.log,
            "Get operation on {:?} returned {} items",
            &store,
            answ.len()
        );
        answ
    }
}

impl StockImpl {
    /// Returns a new instance of `StockImpl` given a
    /// [slog::Logger](https://docs.rs/slog/2.5.2/slog/struct.Logger.html)
    /// .
    pub fn new(log: Logger) -> Self {
        Self {
            stores: collections::HashMap::new(),
            log,
        }
    }

    /// Adds a [`Product`](struct.Product.html) to the specified
    /// [`Store`][0]. It creates the [`Store`][0] storage space if it does
    /// not exist yet.
    ///
    /// [0]: proto/stock/enum.Store.html
    pub fn register(&mut self, pdct: Product, store: proto::Store, qt: u32) {
        if let Some(products) = self.stores.get_mut(&store) {
            debug!(self.log, "Store '{:?}' already present on the stock", store);
            trace!(self.log, "Adding {} of '{:?}' to '{:?}'", qt, &pdct, store);
            products.insert(pdct, qt);
        } else {
            debug!(
                self.log,
                "Store '{:?}' not present, creating it on the stock", store
            );
            let mut products = collections::HashMap::new();
            trace!(self.log, "Adding {} of '{:?}' to '{:?}'", qt, &pdct, store);
            products.insert(pdct, qt);
            self.stores.insert(store, products);
        }
    }
}

/// `Product` is a simple representation of a product.
#[derive(Clone, Debug, cmp::Eq, cmp::PartialEq, hash::Hash)]
pub struct Product {
    name: String,
    brand: String,
    category: String,
}

impl Product {
    /// Returns a instance of itself given a name, brand and category.
    /// All the fields are represented by standard `&str`.
    pub fn new(name: &str, brand: &str, category: &str) -> Self {
        Self {
            name: String::from(name),
            brand: String::from(brand),
            category: String::from(category),
        }
    }
}

#[cfg(test)]
mod tests;
