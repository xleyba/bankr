The way to configure the logs is explained here 
https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html


and should belike this in this case:

RUST_LOG="warn,bank::*=debug" cargo run

if we want to configure application too:

BANK_PORT=9296 BANK_CLIENT_URL=http://172.0.0.1:9596/ RUST_LOG="warn,bank::main=debug,bank::handlers=debug" cargo run