use envconfig::Envconfig;

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
async fn main() -> Result<(), fern::InitError> {

    setup_logger()?;

    dotenv::dotenv().ok();

    let config = config::Config::init_from_env().unwrap();

    return Ok(());
}
