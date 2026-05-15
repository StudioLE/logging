//! Builder for configuring and constructing a [`Logger`].
use crate::prelude::*;

/// Configure and produce a [`Logger`].
#[derive(Debug)]
pub struct LoggerBuilder {
    level: LogLevel,
    targets: Vec<TargetFilter>,
    #[cfg(not(target_arch = "wasm32"))]
    pub(super) writer: Option<SharedWriter>,
}

impl LoggerBuilder {
    /// Create a new [`LoggerBuilder`] with default settings.
    #[must_use]
    pub fn new() -> Self {
        Self {
            level: LogLevel::default(),
            targets: Vec::new(),
            #[cfg(not(target_arch = "wasm32"))]
            writer: None,
        }
    }

    /// Set the default log level.
    #[must_use]
    pub fn with_level(mut self, level: LogLevel) -> Self {
        self.level = level;
        self
    }

    /// Add a per-target log level override.
    #[must_use]
    pub fn with_target(mut self, name: impl Into<String>, level: LogLevel) -> Self {
        self.targets.push(TargetFilter {
            name: name.into(),
            level,
        });
        self
    }

    /// Build the [`Logger`].
    #[must_use]
    pub fn build(self) -> Logger {
        Logger {
            level: self.level,
            targets: self.targets,
            #[cfg(not(target_arch = "wasm32"))]
            writer: self.writer,
        }
    }
}

impl Default for LoggerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logger_builder_defaults() {
        let logger = LoggerBuilder::new().build();
        assert_eq!(logger.level, LogLevel::Info);
        assert!(logger.targets.is_empty());
    }

    #[test]
    fn logger_builder_with_level() {
        let logger = LoggerBuilder::new().with_level(LogLevel::Debug).build();
        assert_eq!(logger.level, LogLevel::Debug);
    }

    #[test]
    fn logger_builder_with_target() {
        let logger = LoggerBuilder::new()
            .with_target("sqlx", LogLevel::Warn)
            .with_target("hyper", LogLevel::Error)
            .build();
        assert_eq!(logger.targets.len(), 2);
        let first = logger.targets.first().expect("first target");
        assert_eq!(first.name, "sqlx");
        assert_eq!(first.level, LogLevel::Warn);
        let second = logger.targets.get(1).expect("second target");
        assert_eq!(second.name, "hyper");
        assert_eq!(second.level, LogLevel::Error);
    }
}
