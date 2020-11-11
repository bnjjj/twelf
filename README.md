# Twelf

Twelf is a configuration solution for Rust including 12-Factor support. It is designed with `Layer`s in order to configure different sources and formats to build your configuration. The main goal is to be very simple using the proc macro `twelf::config`. For now it supports : 

+ Default settings (inside your codebase with `#[serde(default = ...)]` coming from [serde](https://serde.rs))
+ Reading from `TOML`, `YAML`, `JSON`, `DHALL`, `INI` files
+ Reading from environment variables: it supports `HashMap` structure with `MY_VARIABLE="mykey=myvalue,mykey2=myvalue2"` and also array like `MY_VARIABLE=first,second` thanks to [envy](https://github.com/softprops/envy).
+ All [serde](https://serde.rs) attributes can be used in your struct to customize your configuration as you wish
+ Reading your configuration from your command line built with [clap](https://github.com/clap-rs/clap)

# Usage 

## Simple with JSON and environment variables

```rust
use twelf::config;

#[config]
struct TestCfg {
    test: String,
    another: usize,
}

// Init configuration with layers, each layers override only existing fields
let config = TestCfg::with_layers(&[
    Layer::Json("conf.json".into()),
    Layer::Env(Some("PREFIX_"))
]).unwrap();
```

## Example with clap support

```rust
use twelf::config;

#[config]
struct TestCfg {
    /// Here is an example of documentation which is displayed in clap
    test: String,
    another: usize,
}

// Will generate global arguments for each of your fields inside your configuration struct
let app = clap::App::new("test").args(&Conf::clap_args());

// Init configuration with layers, each layers override only existing fields
let config = TestCfg::with_layers(&[
    Layer::Json("conf.json".into()),
    Layer::Env(Some("PREFIX_")),
    Layer::Clap(app.get_matches().clone())
]).unwrap();

// ... your application code
```


# TODO:
+ Add tests for feature flags (ex: cargo test --no-default-features --features dhall --test dhall)
+ Write an example with env + files
+ Write an example with default + env
+ Write an example with clap
+ Write an example with complex structure like hashmap, array, flattened
+ Support Vault
+ Imagine a good trait/api to extend and let users fetch config from remote
+ Refactor to let user extend layers
+ Add support of nested struct in envy
+ Fix issue with `#[serde(flatten)] when you use other type than `String` in sub types


# Alternatives

+ [config-rs](https://github.com/mehcode/config-rs) is almost doing the same except the environment layer (for example we support hashmap and array in environment variables). Also `config-rs` don't have clap support and it didn't use any proc-macros if you're not very fan of proc-macros.