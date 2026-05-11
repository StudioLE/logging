//! Dependency injection registration for [`Logger`](crate::construction::Logger).

mod logger_factory;
mod with_logging;

pub(crate) use logger_factory::*;
pub use with_logging::*;
