use envconfig::Envconfig;
use errors::DirtyError;
use log::error;

#[macro_use]
extern crate envconfig_derive;

mod config;
mod blockchain;
mod errors;

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("bot.log")?)
        .apply()?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), DirtyError> {

    setup_logger().expect("Logger could not be created!");

    dotenv::dotenv().ok();

    let config = config::Config::init_from_env().unwrap();
    let mut blockchain = blockchain::Blockchain::new(&config)?;
    let balance = blockchain.get_balance("0x454FaCBA3caA2aD8F5639317847231efb7fa558C".into()).await?;
    error!("Balance: {}", balance);
    /*blockchain.deposit().await;*/

    return Ok(());
}
