#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;

#[cfg(all(
	any(windows, unix),
	target_arch = "x86_64",
	not(target_env = "musl"),
	not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[napi]
pub fn hello_world() -> &'static str {
	"Hello World"
}
