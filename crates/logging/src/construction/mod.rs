//! Logger construction and builder.

mod logger;
mod logger_builder;
#[cfg(not(target_arch = "wasm32"))]
mod logger_native;
#[cfg(target_arch = "wasm32")]
mod logger_wasm;

pub use logger::*;
pub use logger_builder::*;
