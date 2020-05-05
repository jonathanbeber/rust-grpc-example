use std::{str, sync};

#[macro_use]
extern crate prettytable;
use prettytable::{format, Table};
use structopt::StructOpt;

use stock::grpc::client;
use stock::proto;

/// Opt is the struct responsible for holding the server initialization options.
#[derive(Debug, StructOpt)]
#[structopt(
    name = format!("{}-{}", env!("CARGO_PKG_NAME"), "client"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,

    /// Sets the stock server address.
    #[structopt(long, default_value = "127.0.0.1", global(true))]
    host: String,

    /// Sets the stock server port.
    #[structopt(long, default_value = "9090", global(true))]
    port: u16,

    /// Defines the store where actions will be performed. Currently VENEZA_IT or BERLIN_DE.
    #[structopt(long, short, default_value = "BERLIN_DE", global(true))]
    store: proto::Store,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// List the products available in the store.
    List {
        /// Displays unavailable items too.
        #[structopt(long)]
        show_unavailable: bool,
    },
}

fn print_table(result: proto::StockResponse) {
    let mut table = Table::new();
    let clean = format::FormatBuilder::new().padding(0, 4).build();
    table.set_format(clean);

    table.add_row(row!["PRODUCT", "BRAND", "CATEGORY", "STATUS", "AVAILABLE"]);
    for item in result.items.into_vec() {
        table.add_row(row![
            item.name,
            item.brand,
            item.category,
            item.availability_description,
            item.quantity
        ]);
    }
    table.printstd();
}

fn main() {
    // Parses cli options
    let opt = Opt::from_args();

    let env = sync::Arc::new(grpcio::EnvBuilder::new().build());
    let ch = grpcio::ChannelBuilder::new(env).connect(&format!("{}:{}", &opt.host, &opt.port));
    let client = proto::StockClient::new(ch);

    match opt.cmd {
        Command::List { show_unavailable } => {
            match client.list_items(&client::generate_list_request(opt.store, show_unavailable)) {
                Ok(result) => {
                    println!("Products available on {:?}", &opt.store);
                    print_table(result);
                }
                Err(err) => {
                    eprintln!("gRPC call failed: {}", err);
                    std::process::exit(2);
                }
            }
        }
    };
}
