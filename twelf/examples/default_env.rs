#![allow(dead_code)]

use twelf::{config, Layer};

#[config]
#[derive(Debug)]
struct Config {
    db_host: String,
    threads: usize,
    #[serde(default = "get_default_value")]
    default_value: String,
}

fn get_default_value() -> String {
    String::from("my default value")
}
fn main() {
    std::env::set_var("DB_HOST", "localhost");
    std::env::set_var("THREADS", "5");
    let config = Config::with_layers(&[Layer::Env(None)]).unwrap();

    println!("config - {:?}", config);
}
