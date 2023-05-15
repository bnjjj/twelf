use clap::{CommandFactory, Parser};
use clap_rs as clap;
use twelf::config;
use twelf::Layer;

/// Simple program to greet a person
#[config]
#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    // let args = Args::parse();
    let matches = Args::command().get_matches();
    let config =
        Args::with_layers(&[Layer::Env(Some(String::from("APP_"))), Layer::Clap(matches)]).unwrap();
    println!("Hello {:?}!", config.name)
}
