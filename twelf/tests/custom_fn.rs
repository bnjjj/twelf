#![cfg(feature = "custom_fn")]

use config_derive::config;
use twelf::Layer;

use serde_json::value::to_value;

#[test]
fn simple_custom_fn() {
    #[config]
    #[derive(Debug, Default)]
    struct TestCfg {
        test: String,
        another: usize,
    }

    std::env::set_var("ANOTHER", "5");

    let func = || {
        to_value(TestCfg{test: "from func".to_string(), another: 25}).unwrap()
    };

    let prio = vec![Layer::CustomFn(func.into()), Layer::Env(None)];
    let config = TestCfg::with_layers(&prio).unwrap();
    assert_eq!(config.test, String::from("from func"));
    assert_eq!(config.another, 5usize);

    let prio = vec![Layer::Env(None), Layer::CustomFn(func.into())];
    let config = TestCfg::with_layers(&prio).unwrap();
    assert_eq!(config.test, String::from("from func"));
    assert_eq!(config.another, 25usize);
}
