pub mod blockchain;
pub mod config;
pub mod db;
pub mod errors;
pub mod interface;
pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;

fn _setup_logger() -> Result<(), fern::InitError> {
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
