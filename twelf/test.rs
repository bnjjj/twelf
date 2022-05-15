#![feature(prelude_import)]
#![allow(dead_code)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use clap_rs as clap;
use clap::{Parser, Subcommand, CommandFactory};
use twelf::{config, Layer};
#[serde(crate = "::twelf::reexports::serde")]
# [clap (author , version , about , long_about = None)]
struct Config {
    #[clap(long, help = "Documentation inside clap, to specifiy db_host")]
    db_host: String,
    #[clap(long, short, help = "The number of threads")]
    #[clap(required = false)]
    threads: usize,
    #[clap(long, short, help = "Put in verbose mode")]
    verbose: bool,
}
impl clap::Parser for Config {}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting
)]
#[deny(clippy::correctness)]
#[allow(deprecated)]
impl clap::CommandFactory for Config {
    fn into_app<'b>() -> clap::Command<'b> {
        let __clap_app = clap::Command::new("twelf");
        <Self as clap::Args>::augment_args(__clap_app)
    }
    fn into_app_for_update<'b>() -> clap::Command<'b> {
        let __clap_app = clap::Command::new("twelf");
        <Self as clap::Args>::augment_args_for_update(__clap_app)
    }
}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting
)]
#[deny(clippy::correctness)]
impl clap::FromArgMatches for Config {
    fn from_arg_matches(
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<Self, clap::Error> {
        let v = Config {
            db_host: __clap_arg_matches
                .value_of("db-host")
                .ok_or_else(|| {
                    clap::Error::raw(clap::ErrorKind::MissingRequiredArgument, {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["The following required argument was not provided: "],
                            &[::core::fmt::ArgumentV1::new_display(&"db-host")],
                        ));
                        res
                    })
                })
                .and_then(|s| {
                    ::std::str::FromStr::from_str(s).map_err(|err| {
                        clap::Error::raw(clap::ErrorKind::ValueValidation, {
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Invalid value for ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&"db-host"),
                                    ::core::fmt::ArgumentV1::new_display(&err),
                                ],
                            ));
                            res
                        })
                    })
                })?,
            threads: __clap_arg_matches
                .value_of("threads")
                .ok_or_else(|| {
                    clap::Error::raw(clap::ErrorKind::MissingRequiredArgument, {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["The following required argument was not provided: "],
                            &[::core::fmt::ArgumentV1::new_display(&"threads")],
                        ));
                        res
                    })
                })
                .and_then(|s| {
                    ::std::str::FromStr::from_str(s).map_err(|err| {
                        clap::Error::raw(clap::ErrorKind::ValueValidation, {
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Invalid value for ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&"threads"),
                                    ::core::fmt::ArgumentV1::new_display(&err),
                                ],
                            ));
                            res
                        })
                    })
                })?,
            verbose: __clap_arg_matches.is_present("verbose"),
        };
        ::std::result::Result::Ok(v)
    }
    fn update_from_arg_matches(
        &mut self,
        __clap_arg_matches: &clap::ArgMatches,
    ) -> ::std::result::Result<(), clap::Error> {
        if __clap_arg_matches.is_present("db-host") {
            #[allow(non_snake_case)]
            let db_host = &mut self.db_host;
            *db_host = __clap_arg_matches
                .value_of("db-host")
                .ok_or_else(|| {
                    clap::Error::raw(clap::ErrorKind::MissingRequiredArgument, {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["The following required argument was not provided: "],
                            &[::core::fmt::ArgumentV1::new_display(&"db-host")],
                        ));
                        res
                    })
                })
                .and_then(|s| {
                    ::std::str::FromStr::from_str(s).map_err(|err| {
                        clap::Error::raw(clap::ErrorKind::ValueValidation, {
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Invalid value for ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&"db-host"),
                                    ::core::fmt::ArgumentV1::new_display(&err),
                                ],
                            ));
                            res
                        })
                    })
                })?
        }
        if __clap_arg_matches.is_present("threads") {
            #[allow(non_snake_case)]
            let threads = &mut self.threads;
            *threads = __clap_arg_matches
                .value_of("threads")
                .ok_or_else(|| {
                    clap::Error::raw(clap::ErrorKind::MissingRequiredArgument, {
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["The following required argument was not provided: "],
                            &[::core::fmt::ArgumentV1::new_display(&"threads")],
                        ));
                        res
                    })
                })
                .and_then(|s| {
                    ::std::str::FromStr::from_str(s).map_err(|err| {
                        clap::Error::raw(clap::ErrorKind::ValueValidation, {
                            let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                &["Invalid value for ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&"threads"),
                                    ::core::fmt::ArgumentV1::new_display(&err),
                                ],
                            ));
                            res
                        })
                    })
                })?
        }
        if __clap_arg_matches.is_present("verbose") {
            #[allow(non_snake_case)]
            let verbose = &mut self.verbose;
            *verbose = *verbose || __clap_arg_matches.is_present("verbose")
        }
        ::std::result::Result::Ok(())
    }
}
#[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
#[allow(
    clippy::style,
    clippy::complexity,
    clippy::pedantic,
    clippy::restriction,
    clippy::perf,
    clippy::deprecated,
    clippy::nursery,
    clippy::cargo,
    clippy::suspicious_else_formatting
)]
#[deny(clippy::correctness)]
impl clap::Args for Config {
    fn augment_args<'b>(__clap_app: clap::Command<'b>) -> clap::Command<'b> {
        {
            let __clap_app = __clap_app;
            let __clap_app = __clap_app.arg(
                clap::Arg::new("db-host")
                    .takes_value(true)
                    .value_name("DB_HOST")
                    .required(true)
                    .validator(|s| ::std::str::FromStr::from_str(s).map(|_: String| ()))
                    .long("db-host")
                    .help("Documentation inside clap, to specifiy db_host"),
            );
            let __clap_app = __clap_app.arg(
                clap::Arg::new("threads")
                    .takes_value(true)
                    .value_name("THREADS")
                    .required(true)
                    .validator(|s| ::std::str::FromStr::from_str(s).map(|_: usize| ()))
                    .long("threads")
                    .short('\u{74}')
                    .help("The number of threads")
                    .required(false),
            );
            let __clap_app = __clap_app.arg(
                clap::Arg::new("verbose")
                    .long("verbose")
                    .short('\u{76}')
                    .help("Put in verbose mode"),
            );
            __clap_app . author ("Benjamin Coenen <5719034+bnjjj@users.noreply.github.com>") . version ("0.4.0") . about ("Twelf is a configuration solution for Rust including 12-Factor support. It is designed with layers in order to configure different sources and formats to build your configuration. The main goal is to be very simple using a proc macro.") . long_about (None)
        }
    }
    fn augment_args_for_update<'b>(__clap_app: clap::Command<'b>) -> clap::Command<'b> {
        {
            let __clap_app = __clap_app;
            let __clap_app = __clap_app.arg(
                clap::Arg::new("db-host")
                    .takes_value(true)
                    .value_name("DB_HOST")
                    .required(false)
                    .validator(|s| ::std::str::FromStr::from_str(s).map(|_: String| ()))
                    .long("db-host")
                    .help("Documentation inside clap, to specifiy db_host"),
            );
            let __clap_app = __clap_app.arg(
                clap::Arg::new("threads")
                    .takes_value(true)
                    .value_name("THREADS")
                    .required(false)
                    .validator(|s| ::std::str::FromStr::from_str(s).map(|_: usize| ()))
                    .long("threads")
                    .short('\u{74}')
                    .help("The number of threads")
                    .required(false),
            );
            let __clap_app = __clap_app.arg(
                clap::Arg::new("verbose")
                    .long("verbose")
                    .short('\u{76}')
                    .help("Put in verbose mode"),
            );
            __clap_app . author ("Benjamin Coenen <5719034+bnjjj@users.noreply.github.com>") . version ("0.4.0") . about ("Twelf is a configuration solution for Rust including 12-Factor support. It is designed with layers in order to configure different sources and formats to build your configuration. The main goal is to be very simple using a proc macro.") . long_about (None)
        }
    }
}
#[automatically_derived]
#[allow(unused_qualifications)]
impl ::core::fmt::Debug for Config {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        match *self {
            Config {
                db_host: ref __self_0_0,
                threads: ref __self_0_1,
                verbose: ref __self_0_2,
            } => {
                let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Config");
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "db_host",
                    &&(*__self_0_0),
                );
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "threads",
                    &&(*__self_0_1),
                );
                let _ = ::core::fmt::DebugStruct::field(
                    debug_trait_builder,
                    "verbose",
                    &&(*__self_0_2),
                );
                ::core::fmt::DebugStruct::finish(debug_trait_builder)
            }
        }
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    use ::twelf::reexports::serde as _serde;
    #[automatically_derived]
    impl<'de> ::twelf::reexports::serde::Deserialize<'de> for Config {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> ::twelf::reexports::serde::__private::Result<Self, __D::Error>
        where
            __D: ::twelf::reexports::serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "db_host" => _serde::__private::Ok(__Field::__field0),
                        "threads" => _serde::__private::Ok(__Field::__field1),
                        "verbose" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"db_host" => _serde::__private::Ok(__Field::__field0),
                        b"threads" => _serde::__private::Ok(__Field::__field1),
                        b"verbose" => _serde::__private::Ok(__Field::__field2),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<Config>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = Config;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "struct Config")
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 =
                        match match _serde::de::SeqAccess::next_element::<String>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct Config with 3 elements",
                                ));
                            }
                        };
                    let __field1 =
                        match match _serde::de::SeqAccess::next_element::<usize>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct Config with 3 elements",
                                ));
                            }
                        };
                    let __field2 =
                        match match _serde::de::SeqAccess::next_element::<bool>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct Config with 3 elements",
                                ));
                            }
                        };
                    _serde::__private::Ok(Config {
                        db_host: __field0,
                        threads: __field1,
                        verbose: __field2,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<usize> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<bool> = _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "db_host",
                                        ),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<String>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "threads",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<usize>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "verbose",
                                        ),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<bool>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {
                                let _ = match _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)
                                {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("db_host") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("threads") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("verbose") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(Config {
                        db_host: __field0,
                        threads: __field1,
                        verbose: __field2,
                    })
                }
            }
            const FIELDS: &'static [&'static str] = &["db_host", "threads", "verbose"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "Config",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<Config>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl Config {
    pub fn with_layers(layers: &[::twelf::Layer]) -> Result<Self, ::twelf::Error> {
        use std::iter::FromIterator;
        let mut res: std::collections::HashMap<String, ::twelf::reexports::serde_json::Value> =
            std::collections::HashMap::new();
        for layer in layers {
            let extension = Self::parse_twelf(layer)?;
            res.extend(
                extension
                    .as_object()
                    .ok_or_else(|| ::twelf::Error::InvalidFormat)?
                    .to_owned()
                    .into_iter()
                    .filter(|(_k, v)| !v.is_null()),
            );
        }
        {
            let lvl = ::log::Level::Debug;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(
                    ::core::fmt::Arguments::new_v1(&["configuration:"], &[]),
                    lvl,
                    &(
                        "twelf",
                        "clap_derive",
                        "twelf/examples/clap_derive.rs",
                        7u32,
                    ),
                    ::log::__private_api::Option::None,
                );
            }
        };
        for (key, val) in &res {
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["", "="],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&key),
                                ::core::fmt::ArgumentV1::new_display(&val),
                            ],
                        ),
                        lvl,
                        &(
                            "twelf",
                            "clap_derive",
                            "twelf/examples/clap_derive.rs",
                            7u32,
                        ),
                        ::log::__private_api::Option::None,
                    );
                }
            };
        }
        ::twelf::reexports::serde_json::from_value(::twelf::reexports::serde_json::Value::Object(
            ::twelf::reexports::serde_json::Map::from_iter(res.into_iter()),
        ))
        .map_err(|e| {
            ::twelf::Error::Deserialize({
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["config error: "],
                    &[::core::fmt::ArgumentV1::new_display(&e.to_string())],
                ));
                res
            })
        })
    }
    pub fn clap_args() -> Vec<::twelf::reexports::clap::Arg<'static>> {
        <[_]>::into_vec(box [
            ::twelf::reexports::clap::Arg::new("db-host")
                .long("db-host")
                .help("")
                .takes_value(!false)
                .global(true),
            ::twelf::reexports::clap::Arg::new("threads")
                .long("threads")
                .help("")
                .takes_value(!false)
                .global(true),
            ::twelf::reexports::clap::Arg::new("verbose")
                .long("verbose")
                .help("")
                .takes_value(!true)
                .global(true),
        ])
    }
    fn parse_twelf(
        priority: &::twelf::Layer,
    ) -> Result<::twelf::reexports::serde_json::Value, ::twelf::Error> {
        #[serde(crate = "::twelf::reexports::serde")]
        struct OptConfig {
            db_host: Option<String>,
            threads: Option<usize>,
            verbose: Option<bool>,
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            use ::twelf::reexports::serde as _serde;
            #[automatically_derived]
            impl<'de> ::twelf::reexports::serde::Deserialize<'de> for OptConfig {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> ::twelf::reexports::serde::__private::Result<Self, __D::Error>
                where
                    __D: ::twelf::reexports::serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    struct __FieldVisitor;
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "field identifier")
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "db_host" => _serde::__private::Ok(__Field::__field0),
                                "threads" => _serde::__private::Ok(__Field::__field1),
                                "verbose" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"db_host" => _serde::__private::Ok(__Field::__field0),
                                b"threads" => _serde::__private::Ok(__Field::__field1),
                                b"verbose" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<OptConfig>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = OptConfig;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(__formatter, "struct OptConfig")
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match match _serde::de::SeqAccess::next_element::<
                                Option<String>,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct OptConfig with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match match _serde::de::SeqAccess::next_element::<
                                Option<usize>,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct OptConfig with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match match _serde::de::SeqAccess::next_element::<
                                Option<bool>,
                            >(&mut __seq)
                            {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct OptConfig with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(OptConfig {
                                db_host: __field0,
                                threads: __field1,
                                verbose: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<Option<String>> =
                                _serde::__private::None;
                            let mut __field1: _serde::__private::Option<Option<usize>> =
                                _serde::__private::None;
                            let mut __field2: _serde::__private::Option<Option<bool>> =
                                _serde::__private::None;
                            while let _serde::__private::Some(__key) =
                                match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "db_host",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<Option<String>>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "threads",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<Option<usize>>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "verbose",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            match _serde::de::MapAccess::next_value::<Option<bool>>(
                                                &mut __map,
                                            ) {
                                                _serde::__private::Ok(__val) => __val,
                                                _serde::__private::Err(__err) => {
                                                    return _serde::__private::Err(__err);
                                                }
                                            },
                                        );
                                    }
                                    _ => {
                                        let _ = match _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(
                                            &mut __map
                                        ) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        };
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("db_host") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("threads") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    match _serde::__private::de::missing_field("verbose") {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    }
                                }
                            };
                            _serde::__private::Ok(OptConfig {
                                db_host: __field0,
                                threads: __field1,
                                verbose: __field2,
                            })
                        }
                    }
                    const FIELDS: &'static [&'static str] = &["db_host", "threads", "verbose"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "OptConfig",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<OptConfig>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[doc(hidden)]
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            use ::twelf::reexports::serde as _serde;
            #[automatically_derived]
            impl ::twelf::reexports::serde::Serialize for OptConfig {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> ::twelf::reexports::serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: ::twelf::reexports::serde::Serializer,
                {
                    let mut __serde_state = match _serde::Serializer::serialize_struct(
                        __serializer,
                        "OptConfig",
                        false as usize + 1 + 1 + 1,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "db_host",
                        &self.db_host,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "threads",
                        &self.threads,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    match _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "verbose",
                        &self.verbose,
                    ) {
                        _serde::__private::Ok(__val) => __val,
                        _serde::__private::Err(__err) => {
                            return _serde::__private::Err(__err);
                        }
                    };
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        let res = match priority {
            ::twelf::Layer::Env(prefix) => match prefix {
                Some(prefix) => {
                    let tmp_cfg: OptConfig =
                        ::twelf::reexports::envy::prefixed(prefix).from_env()?;
                    ::twelf::reexports::serde_json::to_value(tmp_cfg)
                }
                None => {
                    let tmp_cfg: OptConfig = ::twelf::reexports::envy::from_env()?;
                    ::twelf::reexports::serde_json::to_value(tmp_cfg)
                }
            }?,
            ::twelf::Layer::Json(filepath) => {
                ::twelf::reexports::serde_json::from_reader(std::fs::File::open(filepath)?)?
            }
            ::twelf::Layer::Toml(filepath) => {
                ::twelf::reexports::toml::from_str(&std::fs::read_to_string(filepath)?)?
            }
            ::twelf::Layer::Yaml(filepath) => {
                ::twelf::reexports::serde_yaml::from_str(&std::fs::read_to_string(filepath)?)?
            }
            ::twelf::Layer::Dhall(filepath) => {
                ::twelf::reexports::serde_dhall::from_str(&std::fs::read_to_string(filepath)?)
                    .parse()?
            }
            ::twelf::Layer::Ini(filepath) => {
                let tmp_cfg: OptConfig =
                    ::twelf::reexports::serde_ini::from_str(&std::fs::read_to_string(filepath)?)?;
                ::twelf::reexports::serde_json::to_value(tmp_cfg)?
            }
            ::twelf::Layer::Clap(matches) => {
                let mut map: std::collections::HashMap<String, String> =
                    std::collections::HashMap::new();
                if let Some(vmatch) = matches.value_of("db-host") {
                    map.insert(String::from("db_host"), vmatch.to_string());
                } else if false {
                    if matches.is_present("db-host") {
                        map.insert(String::from("db_host"), String::from("true"));
                    }
                }
                if let Some(vmatch) = matches.value_of("threads") {
                    map.insert(String::from("threads"), vmatch.to_string());
                } else if false {
                    if matches.is_present("threads") {
                        map.insert(String::from("threads"), String::from("true"));
                    }
                }
                if let Some(vmatch) = matches.value_of("verbose") {
                    map.insert(String::from("verbose"), vmatch.to_string());
                } else if true {
                    if matches.is_present("verbose") {
                        map.insert(String::from("verbose"), String::from("true"));
                    }
                }
                let tmp_cfg: OptConfig = ::twelf::reexports::envy::from_iter(map.into_iter())?;
                ::twelf::reexports::serde_json::to_value(tmp_cfg)?
            }
            other => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["not implemented: "],
                &[::core::fmt::ArgumentV1::new_display(
                    &::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_debug(&other)],
                    ),
                )],
            )),
        };
        Ok(res)
    }
}
fn main() {
    let matches = Config::command().ignore_errors(true).get_matches();
    let config =
        Config::with_layers(&[Layer::Env(Some(String::from("APP_"))), Layer::Clap(matches)])
            .unwrap();
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1(
            &["config - ", "\n"],
            &[::core::fmt::ArgumentV1::new_debug(&config)],
        ));
    };
}
