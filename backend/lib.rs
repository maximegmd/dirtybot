#[macro_use]
extern crate diesel;
#[macro_use]
extern crate napi_derive;

mod blockchain;
pub mod context;
mod errors;
mod models;
mod schema;

#[cfg(all(
	any(windows, unix),
	target_arch = "x86_64",
	not(target_env = "musl"),
	not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

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
		.apply()?;
	Ok(())
}
