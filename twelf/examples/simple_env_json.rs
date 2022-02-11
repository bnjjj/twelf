use twelf::{config, Layer};

#[config]
#[derive(Debug)]
struct Config {
    #[allow(dead_code)]
    db_host: String,

    #[allow(dead_code)]
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
