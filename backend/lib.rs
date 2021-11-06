#[macro_use]
extern crate diesel;
#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;
use tinyjson::JsonValue;

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

#[napi]
pub fn setup_logger() -> Result<()> {
	let hostname = JsonValue::String(hostname::get()?.into_string().unwrap())
		.stringify()
		.unwrap();

	let pid = std::process::id();

	fern::Dispatch::new()
		.format(move |out, message, record| {
			out.finish(format_args!(
				"{{\"level\":{},\"time\":{},\"msg\":{},\"pid\":{},\"hostname\":{}}}",
				60 - (record.level() as usize) * 10,
				chrono::Utc::now().timestamp_millis(),
				JsonValue::String(message.to_string()).stringify().unwrap(),
				pid,
				hostname,
			))
		})
		.level(log::LevelFilter::Debug)
		.chain(std::io::stdout())
		.apply()
		.map_err(|e| Error::from_reason(e.to_string()))?;

	Ok(())
}
