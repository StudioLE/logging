//! Logging registration sugar on [`ServiceBuilder`].
use crate::prelude::*;

/// Extend [`ServiceBuilder`] with logging registration.
pub trait WithLogging {
    /// Register a [`Logger`] service configured by the given factory function.
    ///
    /// The factory receives the [`ServiceProvider`] and returns a [`Logger`]
    /// constructed via [`LoggerBuilder`]. The logger is initialized during
    /// [`ServiceProvider::init`].
    #[must_use]
    fn with_logging(
        self,
        factory: fn(&ServiceProvider) -> Result<Logger, Report<ResolveError>>,
    ) -> Self;
}

impl WithLogging for ServiceBuilder {
    fn with_logging(
        self,
        factory: fn(&ServiceProvider) -> Result<Logger, Report<ResolveError>>,
    ) -> Self {
        self.with_instance(LoggerFactory::new(factory))
            .with_type::<Logger>()
            .with_init::<Logger>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AppCliOptions {
        pub log_level: LogLevel,
    }

    #[test]
    fn service_builder_with_logging_init() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_instance(AppCliOptions {
                log_level: LogLevel::Info,
            })
            .with_logging(|services| {
                let cli = services.get::<AppCliOptions>()?;
                Ok(LoggerBuilder::new().with_level(cli.log_level).build())
            })
            .build();
        // Act
        let output = services.init();
        // Assert
        assert!(output.is_ok());
    }

    /// Call [`ServiceProvider::init`] from a second test in the same process.
    ///
    /// - Exercises the [`OnceLock`] guard in [`Logger::init`]
    /// - Reproduces the scenario where parallel tests both build a service
    ///   container that registers logging and call init
    #[test]
    fn service_builder_with_logging_init_twice() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_instance(AppCliOptions {
                log_level: LogLevel::Info,
            })
            .with_logging(|services| {
                let cli = services.get::<AppCliOptions>()?;
                Ok(LoggerBuilder::new().with_level(cli.log_level).build())
            })
            .build();
        // Act
        let output = services.init();
        // Assert
        assert!(output.is_ok());
    }

    #[test]
    fn service_builder_with_logging() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_instance(AppCliOptions {
                log_level: LogLevel::Trace,
            })
            .with_logging(|services| {
                let cli = services.get::<AppCliOptions>()?;
                Ok(LoggerBuilder::new()
                    .with_level(cli.log_level)
                    .with_target("noisy_crate", LogLevel::Warn)
                    .build())
            })
            .build();
        // Act
        let logger = services.get::<Logger>();
        // Assert
        assert!(logger.is_ok());
        assert_eq!(logger.expect("should resolve").level, LogLevel::Trace);
    }
}
