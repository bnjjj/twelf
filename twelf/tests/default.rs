#![cfg(features = "default_trait")]
use config_derive::config;
use twelf::Layer;

#[test]
fn default_simple_types() {
    #[config]
    #[derive(Debug)]
    struct TestCfg {
        test: String,
        another: usize,
    }

    impl Default for TestCfg {
        fn default() -> Self {
            Self {
                test: "from default".to_owned(),
                another: 25,
            }
        }
    }

    std::env::set_var("ANOTHER", "5");

    let prio = vec![Layer::DefaultTrait, Layer::Env(None)];
    let config = TestCfg::with_layers(&prio).unwrap();
    assert_eq!(config.test, String::from("from default"));
    assert_eq!(config.another, 5usize);

    let prio = vec![Layer::Env(None), Layer::DefaultTrait];
    let config = TestCfg::with_layers(&prio).unwrap();
    assert_eq!(config.test, String::from("from default"));
    assert_eq!(config.another, 25usize);
}
