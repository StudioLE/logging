//! Factory for constructing a [`Logger`] from the DI container.
use crate::prelude::*;

/// Factory that produces a [`Logger`] by resolving dependencies from a [`ServiceProvider`].
pub(crate) struct LoggerFactory {
    /// Factory function receiving the service provider.
    pub factory: fn(&ServiceProvider) -> Result<Logger, Report<ResolveError>>,
}

impl LoggerFactory {
    /// Create a new [`LoggerFactory`] from a factory function.
    pub fn new(factory: fn(&ServiceProvider) -> Result<Logger, Report<ResolveError>>) -> Self {
        Self { factory }
    }
}

impl FromServices for Logger {
    type Error = ResolveError;

    fn from_services(services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        let factory = services.get::<LoggerFactory>()?;
        (factory.factory)(services)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestConfig {
        pub level: LogLevel,
    }

    #[test]
    fn logger_resolves_via_factory() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_instance(TestConfig {
                level: LogLevel::Debug,
            })
            .with_instance(LoggerFactory::new(|services| {
                let config = services.get::<TestConfig>()?;
                Ok(LoggerBuilder::new().with_level(config.level).build())
            }))
            .with_type::<Logger>()
            .build();
        // Act
        let logger = services.get::<Logger>();
        // Assert
        assert!(logger.is_ok());
        assert_eq!(logger.expect("should resolve").level, LogLevel::Debug);
    }

    #[test]
    fn end_to_end_with_logging_sugar() {
        // Arrange
        struct CliOptions {
            pub log_level: LogLevel,
        }
        let services = ServiceBuilder::new()
            .with_instance(CliOptions {
                log_level: LogLevel::Warn,
            })
            .with_logging(|services| {
                let cli = services.get::<CliOptions>()?;
                Ok(LoggerBuilder::new()
                    .with_level(cli.log_level)
                    .with_target("sqlx", LogLevel::Error)
                    .with_target("hyper", LogLevel::Error)
                    .build())
            })
            .build();
        // Act
        let logger = services.get::<Logger>().expect("should resolve");
        // Assert
        assert_eq!(logger.level, LogLevel::Warn);
        assert_eq!(logger.targets.len(), 2);
    }
}
