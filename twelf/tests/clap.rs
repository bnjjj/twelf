#![cfg(feature = "clap")]

use std::collections::HashMap;

use clap_rs as clap;
use clap::{CommandFactory, Parser};
use config_derive::config;
use twelf::Layer;

const JSON_TEST_FILE: &str = "./tests/fixtures/test.json";

#[test]
fn clap_with_array_and_hashmap() {
    #[config]
    #[derive(Debug, Default)]
    struct Conf {
        /// My custom documentation on elements-snake in clap
        elements_snake: HashMap<String, String>,
        arrays: Vec<String>,
    }
    let clap_args = Conf::clap_args();
    let mut app = clap::Command::new("test").args(&clap_args);
    let mut help_msg = vec![];

    app.write_long_help(&mut help_msg).unwrap();

    assert!(std::str::from_utf8(&help_msg)
        .unwrap()
        .contains("My custom documentation on elements-snake in clap"));

    let matches = app.get_matches_from(&[
        "test",
        "--arrays=first,second",
        "--elements-snake=coucou=toi,hello=you",
    ]);

    let prio = vec![
        Layer::Json(JSON_TEST_FILE.into()),
        Layer::Clap(matches),
        Layer::Env(None),
    ];
    let config = Conf::with_layers(&prio).unwrap();
    let mut map = HashMap::new();
    map.insert("coucou".to_string(), "toi".to_string());
    map.insert("hello".to_string(), "you".to_string());

    assert_eq!(config.elements_snake, map);
    let array = vec![String::from("first"), String::from("second")];
    assert_eq!(config.arrays, array);
}

#[test]
fn clap_derive_array() {
    #[config]
    #[derive(Parser,Debug, Default)]
    #[clap(author, version, about, long_about = None)]
    struct Conf {
        #[clap(long, default_value_t = 55)]
        some_val: u32,

        #[clap(long)]
        arrays: Vec<String>,
    }

    let matches = Conf::command().get_matches_from(&[
        "test",
        "--arrays=asdf",
        "--some-val=14",
        "--arrays=qwer,zxc"
    ]);

    let prio = vec![
        Layer::Clap(matches),
    ];
    let config = Conf::with_layers(&prio).unwrap();

    assert_eq!(config.arrays, vec!["asdf","qwer","zxc"]);
}
