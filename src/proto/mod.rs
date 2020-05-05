use std::{str, string};

pub mod stock;
pub mod stock_grpc;

pub use self::stock::AvailabilityDescription;
pub use self::stock::Item;
pub use self::stock::StockRequest;
pub use self::stock::StockResponse;
pub use self::stock::Store;
pub use self::stock_grpc::create_stock as create_stock_service;
pub use self::stock_grpc::Stock as StockService;
pub use self::stock_grpc::StockClient;

impl str::FromStr for Store {
    type Err = String;

    fn from_str(s: &str) -> Result<Store, Self::Err> {
        match s {
            "BERLIN_DE" => Ok(Store::BERLIN_DE),
            "VENEZA_IT" => Ok(Store::VENEZA_IT),
            _ => Err(String::from("Invalid store")),
        }
    }
}

impl string::ToString for AvailabilityDescription {
    fn to_string(&self) -> String {
        match self {
            Self::AVAILABLE => return String::from("AVAILABLE"),
            Self::UNAVAILABLE => return String::from("UNAVAILABLE"),
        }
    }
}
