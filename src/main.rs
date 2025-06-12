mod core;
mod net;

use net::{Client, Server};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: cargo run -- [server | client]");
    }

    match args[1].as_str() {
        "server" => Server::new().run(),
        "client" => Client::new().run(),
        _ => panic!("Mode should be either 'server' or 'client'."),
    }
}
