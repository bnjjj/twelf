#![allow(dead_code)]

use clap_rs as clap;
use twelf::{config, Layer};

#[config]
#[derive(Debug)]
struct Config {
    /// Documentation inside clap, to specifiy db_host
    db_host: String,
    threads: usize,
    /// Put in verbose mode
    verbose: bool,
}

// execute `cargo run --example clap -- --help` to display help and documentation
// execute `cargo run --example clap -- --db-host localhost --threads 5` to set configuration
fn main() {
    let matches = clap::Command::new("test_clap")
        .args(&Config::clap_args())
        .get_matches();
    let config =
        Config::with_layers(&[Layer::Env(Some(String::from("APP_"))), Layer::Clap(matches)])
            .unwrap();

    println!("config - {:?}", config);
}
