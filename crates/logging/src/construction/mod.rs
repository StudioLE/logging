//! Logger construction and builder.

mod logger;
mod logger_builder;
#[cfg(not(target_arch = "wasm32"))]
mod logger_builder_native;
#[cfg(not(target_arch = "wasm32"))]
mod logger_native;
#[cfg(target_arch = "wasm32")]
mod logger_wasm;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod shared_writer;

pub use logger::*;
pub use logger_builder::*;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use shared_writer::*;
