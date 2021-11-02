use envconfig::Envconfig;

#[macro_use]
extern crate envconfig_derive;

mod config;
mod blockchain;
mod errors;

#[tokio::main]
async fn main() {

    dotenv::dotenv().ok();

    let config = config::Config::init_from_env().unwrap();

    println!("Hello, world!");

    return ();
}
