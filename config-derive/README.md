# Twelf

![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)
![Rust](https://github.com/bnjjj/twelf/workflows/Rust/badge.svg)
[![Version](https://img.shields.io/crates/v/twelf.svg)](https://crates.io/crates/twelf)
[![Docs.rs](https://docs.rs/twelf/badge.svg)](https://docs.rs/twelf)

> Twelf is a configuration solution for Rust including 12-Factor support. It is designed with `Layer`s in order to configure different sources and formats to build your configuration. The main goal is to be very simple using the proc macro `twelf::config`.

For now it supports :

- Default settings (inside your codebase with `#[serde(default = ...)]` coming from [serde](https://serde.rs))
- Reading from `TOML`, `YAML`, `JSON`, `DHALL`, `INI` files
- Reading from environment variables: it supports `HashMap` structure with `MY_VARIABLE="mykey=myvalue,mykey2=myvalue2"` and also array like `MY_VARIABLE=first,second` thanks to [envy](https://github.com/softprops/envy).
- All [serde](https://serde.rs) attributes can be used in your struct to customize your configuration as you wish
- Reading your configuration from your command line built with [clap](https://github.com/clap-rs/clap) (ATTENTION: if you're using version < v3 use the `twelf@1.8` version)

# Usage

## Simple with JSON and environment variables

```rust,no_run
use twelf::{config, Layer};

#[config]
struct Conf {
    test: String,
    another: usize,
}

// Init configuration with layers, each layers override only existing fields
let config = Conf::with_layers(&[
    Layer::Json("conf.json".into()),
    Layer::Env(Some("PREFIX_".to_string()))
]).unwrap();
```

## Example with clap support

```rust
use twelf::{config, Layer};

#[config]
struct Conf {
    /// Here is an example of documentation which is displayed in clap
    test: String,
    another: usize,
}

// Will generate global arguments for each of your fields inside your configuration struct
let app = clap::App::new("test").args(&Conf::clap_args());

// Init configuration with layers, each layers override only existing fields
let config = Conf::with_layers(&[
    Layer::Json("conf.json".into()),
    Layer::Env(Some("PREFIX_".to_string())),
    Layer::Clap(app.get_matches().clone())
]).unwrap();

// ... your application code
```

Check [here](./twelf/examples) for more examples.

## Features

Twelf supports crate features, if you only want support for `json`, `env` and `toml` then you just have to add this to your `Cargo.toml`

```toml
twelf = { version = "0.3", default-features = false, features = ["json", "toml", "env"] }
```

Default features are `["env", "dhall", "clap", "ini", "json", "yaml", "toml"]`

# Alternatives

- [config-rs](https://github.com/mehcode/config-rs) is almost doing the same except the environment layer (for example we support hashmap and array in environment variables). Also `config-rs` don't have clap support and it didn't use any proc-macros if you're not very fan of proc-macros.
