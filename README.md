# Twelve




# TODO:
+ Add method to convert env variable to create clap::Arg
+ Add support of nested struct in envy
+ Fix issue with `#[serde(flatten)] when you use other type than `String` in sub types
+ Write an example with env + files
+ Write an example with default + env
+ Write an example with clap
+ Write an example with complex structure like hashmap, array, flattened
+ Support Vault
+ Imagine a good trait/api to extend and let users fetch config from remote

## OPTIONAL:
+ Refactor to let user extend priority list

# Alternatives

+ [config-rs](https://github.com/mehcode/config-rs) is almost doing the same without the same handling for environment layer (for example we support hashmap and array in environment variables). Also `config-rs` don't have proc-macro pattern if you're not very fan of proc-macro system.