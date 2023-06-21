#![allow(dead_code)]

use std::collections::HashMap;

use config_derive::config;
use twelf::Layer;

use clap::{CommandFactory, Parser};
use clap_rs as clap;

fn default_array() -> Vec<String> {
    vec!["first".to_owned(), "second".to_owned()]
}

const TOML_TEST_FILE: &str = "./tests/fixtures/test.toml";
const JSON_TEST_FILE: &str = "./tests/fixtures/test.json";

#[test]
fn missing_values() {
    #[config]
    #[derive(Debug, Default)]
    struct TestCfg {
        test: String,
        another: usize,
    }
    let prio = vec![Layer::Env(None)];
    let config = TestCfg::with_layers(&prio);
    assert!(config.is_err());
}

#[test]
fn simple_only_env() {
    #[config]
    #[derive(Debug, Default)]
    struct TestCfg {
        test: String,
        another: usize,
    }
    let prio = vec![Layer::Env(Some("TEST_".to_string()))];
    std::env::set_var("TEST_ANOTHER", "5");
    std::env::set_var("TEST_TEST", "hello");
    let config = TestCfg::with_layers(&prio).unwrap();
    assert_eq!(config.test, String::from("hello"));
    assert_eq!(config.another, 5usize);
}

#[test]
fn nested_only_env() {
    #[derive(Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    struct Another {
        inner: String,
        // second: u8,
    }
    #[config]
    #[derive(Debug, Default)]
    struct TestCfg {
        test: String,
        #[serde(flatten)]
        another: Another,
    }
    let prio = vec![Layer::Env(Some("NESTED_".to_string()))];
    std::env::set_var("NESTED_INNER", "inner_value");
    std::env::set_var("NESTED_TEST", "hello");
    let cfg: TestCfg = envy::prefixed("NESTED_").from_env().unwrap();
    println!("----- {:?}", cfg);
    let config = TestCfg::with_layers(&prio).unwrap();
    assert_eq!(config.test, String::from("hello"));
    assert_eq!(
        config.another,
        Another {
            inner: String::from("inner_value"),
            // second: 25,
        }
    );
}

#[test]
fn mixed_simple_types() {
    #[config]
    #[derive(Debug, Default)]
    struct TestCfg {
        test: String,
        another: usize,
    }
    std::env::set_var("ANOTHER", "5");
    let prio = vec![
        Layer::Toml(TOML_TEST_FILE.into()),
        Layer::Json(JSON_TEST_FILE.into()),
        Layer::Env(None),
    ];
    let config = TestCfg::with_layers(&prio).unwrap();
    assert_eq!(config.test, String::from("from file"));
    assert_eq!(config.another, 5usize);

    let prio = vec![
        Layer::Env(None),
        Layer::Toml(TOML_TEST_FILE.into()),
        Layer::Json(JSON_TEST_FILE.into()),
    ];
    let config = TestCfg::with_layers(&prio).unwrap();
    assert_eq!(config.test, String::from("from file"));
    assert_eq!(config.another, 25usize);
}

#[test]
fn mixed_simple_with_prefix() {
    #[config]
    #[derive(Debug, Default)]
    struct Conf {
        elements_def: HashMap<String, String>,
        #[serde(default = "default_array")]
        array_def: Vec<String>,
    }

    std::env::set_var("APP_ELEMENTS_DEF", "coucou=toi,hello=you");
    let prio = vec![
        Layer::Toml(TOML_TEST_FILE.into()),
        Layer::Json(JSON_TEST_FILE.into()),
        Layer::Env(Some("APP_".to_string())),
    ];
    let config = Conf::with_layers(&prio).unwrap();
    let mut map = HashMap::new();
    map.insert("coucou".to_string(), "toi".to_string());
    map.insert("hello".to_string(), "you".to_string());

    assert_eq!(config.elements_def, map);
    let array = vec![String::from("first"), String::from("second")];
    assert_eq!(config.array_def, array);
}

#[test]
fn mixed_simple_with_option() {
    #[config]
    #[derive(Debug, Default)]
    struct Conf {
        elements_def: Option<HashMap<String, String>>,
        array_def: Option<Vec<String>>,
    }

    std::env::set_var("APPBIS_ELEMENTS_DEF", "coucou=toi,hello=you");
    let prio = vec![
        Layer::Toml(TOML_TEST_FILE.into()),
        Layer::Json(JSON_TEST_FILE.into()),
        Layer::Env(Some("APPBIS_".to_string())),
    ];
    let config = Conf::with_layers(&prio).unwrap();
    let mut map = HashMap::new();
    map.insert("coucou".to_string(), "toi".to_string());
    map.insert("hello".to_string(), "you".to_string());

    assert_eq!(config.elements_def, Some(map));
    assert_eq!(config.array_def, None);
}

#[test]
fn mixed_with_array_and_hashmap_string() {
    #[config]
    #[derive(Debug, Default)]
    struct Conf {
        elements: HashMap<String, String>,
        #[serde(default = "default_array")]
        array: Vec<String>,
    }

    std::env::set_var("ARRAY", "test,test2");
    std::env::set_var("ELEMENTS", "coucou=toi,hello=you");
    let prio = vec![
        Layer::Toml(TOML_TEST_FILE.into()),
        Layer::Json(JSON_TEST_FILE.into()),
        Layer::Env(None),
    ];
    let config = Conf::with_layers(&prio).unwrap();
    let mut map = HashMap::new();
    map.insert("coucou".to_string(), "toi".to_string());
    map.insert("hello".to_string(), "you".to_string());

    assert_eq!(config.elements, map);
    let array = vec![String::from("test"), String::from("test2")];
    assert_eq!(config.array, array);

    let mut map = HashMap::new();
    map.insert("key".to_string(), "value".to_string());
    map.insert("key2".to_string(), "value2".to_string());

    let prio = vec![
        Layer::Env(None),
        Layer::Toml(TOML_TEST_FILE.into()),
        Layer::Json(JSON_TEST_FILE.into()),
    ];
    let config = Conf::with_layers(&prio).unwrap();
    assert_eq!(config.elements, map);
    let array = vec![String::from("test"), String::from("test2")];
    assert_eq!(config.array, array);
}

#[test]
fn mixed_with_array_and_hashmap_with_default() {
    #[config]
    #[derive(Debug, Default)]
    struct Conf {
        elements_def: HashMap<String, String>,
        #[serde(default = "default_array")]
        array_def: Vec<String>,
    }

    std::env::set_var("ELEMENTS_DEF", "coucou=toi,hello=you");
    let prio = vec![
        Layer::Toml(TOML_TEST_FILE.into()),
        Layer::Json(JSON_TEST_FILE.into()),
        Layer::Env(None),
    ];
    let config = Conf::with_layers(&prio).unwrap();
    let mut map = HashMap::new();
    map.insert("coucou".to_string(), "toi".to_string());
    map.insert("hello".to_string(), "you".to_string());

    assert_eq!(config.elements_def, map);
    let array = vec![String::from("first"), String::from("second")];
    assert_eq!(config.array_def, array);
}

#[test]
fn mixed_clap_defaults() {
    #[config]
    #[derive(Parser, Debug, Default)]
    #[clap(author, version, about, long_about = None)]
    struct Conf {
        #[clap(long, default_value_t = 55)]
        some_num: usize,

        #[clap(long, default_value = "some_val")]
        string_def: String,

        #[clap(long, default_value = "some,values,and,more")]
        array_def: Vec<String>,
    }

    let matches = Conf::command().get_matches_from(&["test", "--array-def=asdf,qwer"]);

    std::env::set_var("STRING_DEF", "coucou toi");

    let prio = vec![
        Layer::Env(None),
        Layer::Clap(matches),
    ];
    let config = Conf::with_layers(&prio).unwrap();

    let vec = vec!["asdf", "qwer"];

    assert_eq!(config.string_def, "coucou toi"); // Env Layer
    assert_eq!(config.some_num, 55);             // Clap default
    assert_eq!(config.array_def, vec);           // Clap CLI
}
