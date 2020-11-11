use std::collections::HashMap;

use twelf::reexports::serde::{Deserialize, Serialize};
use twelf::{config, Layer};

#[config]
#[derive(Debug)]
struct Config {
    list: Vec<String>,
    labels: HashMap<String, String>,
    #[serde(flatten)]
    nested: Nested,
}

#[derive(Debug, Deserialize, Serialize)]
struct Nested {
    inner: String,
}
fn main() {
    std::env::set_var("APP_INNER", "inner value");
    std::env::set_var("APP_LIST", "value1,value2");
    std::env::set_var("APP_LABELS", "key=value, key2=value2");
    let config = Config::with_layers(&[
        Layer::Json("./twelf/examples/config.json".into()),
        Layer::Env(Some(String::from("APP_"))),
    ])
    .unwrap();

    println!("config - {:?}", config);
}
