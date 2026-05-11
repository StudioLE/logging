//! Logger service that initializes the global tracing subscriber.
use crate::prelude::*;
use std::io::stderr;
use tracing::subscriber::set_global_default;
use tracing_subscriber::Layer;
use tracing_subscriber::Registry;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;

/// Logger service providing tracing subscriber initialization.
#[derive(Debug)]
pub struct Logger {
    /// Default log level.
    pub(crate) level: LogLevel,
    /// Per-target level overrides.
    pub(crate) targets: Vec<TargetFilter>,
}

impl Init for Logger {
    fn init(&self, _services: &ServiceProvider) -> Result<(), Report<InitError>> {
        let filter = self.build_targets();
        let subscriber = layer()
            .compact()
            .with_writer(stderr)
            .with_target(false)
            .with_timer(ElapsedTime::default())
            .with_filter(filter);
        set_global_default(Registry::default().with(subscriber)).change_context(InitError::Init)
    }
}

impl Logger {
    /// Build the [`Targets`] filter from level and per-target overrides.
    fn build_targets(&self) -> Targets {
        let mut targets = Targets::new().with_default(LevelFilter::from(self.level));
        for filter in &self.targets {
            targets = targets.with_target(&filter.name, LevelFilter::from(filter.level));
        }
        targets
    }
}
