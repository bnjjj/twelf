use twelf::{config, Layer};

#[config]
#[derive(Debug)]
struct Config {
    #[allow(dead_code)]
    db_host: String,

    #[allow(dead_code)]
    threads: usize,

    #[serde(default = "get_default_value")]
    #[allow(dead_code)]
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
