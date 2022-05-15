#![allow(dead_code)]

use clap::{CommandFactory, Parser};
use clap_rs as clap;
use twelf::{config, Layer};

#[config]
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Config {
    #[clap(long, help = "Documentation inside clap, to specifiy db_host")]
    db_host: String,
    #[clap(long, short, help = "The number of threads")]
    #[clap(required = false, default_value_t = 55)]
    threads: usize,
    #[clap(long, short, help = "Put in verbose mode")]
    verbose: bool,
}

// execute `cargo run --example clap_derive -- --help` to display help and documentation
// execute `cargo run --example clap_derive -- --db-host localhost --threads 5` to set configuration
fn main() {
    let matches = Config::command().ignore_errors(true).get_matches();
    let config =
        Config::with_layers(&[Layer::Env(Some(String::from("APP_"))), Layer::Clap(matches)])
            .unwrap();

    println!("config - {:?}", config);
}
