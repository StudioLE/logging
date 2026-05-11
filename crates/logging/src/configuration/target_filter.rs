//! Per-target log level override.
use crate::prelude::*;

/// Override the log level for a specific target.
///
/// - Target is a crate or module path
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TargetFilter {
    /// Target name.
    ///
    /// - Typically a crate name like `"sqlx"`
    pub name: String,
    /// Log level to apply to this target.
    pub level: LogLevel,
}
