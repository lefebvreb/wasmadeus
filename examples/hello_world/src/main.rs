use wasmadeus::prelude::*;

fn main() {
    ConsoleLogger::new().init().unwrap();
    log::info!("Hello, world!");
}
