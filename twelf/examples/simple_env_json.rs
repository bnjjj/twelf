#![allow(dead_code)]

use twelf::{config, Layer};

#[config]
#[derive(Debug, Default)]
struct Config {
    db_host: String,
    threads: usize,
}
fn main() {
    let config = Config::with_layers(&[
        Layer::Json("./twelf/examples/config.json".into()),
        Layer::Env(Some(String::from("APP_"))),
    ])
    .unwrap();

    println!("config - {:?}", config);
}
