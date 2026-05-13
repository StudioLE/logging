//! Log level, target filtering, and elapsed time configuration.

#[cfg(not(target_arch = "wasm32"))]
mod elapsed_time;
mod log_level;
mod target_filter;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) use elapsed_time::*;
pub use log_level::*;
pub use target_filter::*;
