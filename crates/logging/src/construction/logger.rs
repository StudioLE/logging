//! Logger service that initializes the global tracing subscriber.
use crate::prelude::*;
#[cfg(not(feature = "testing"))]
use std::io::stderr;
use std::sync::OnceLock;
use tracing::subscriber::set_global_default;
use tracing_subscriber::Layer;
use tracing_subscriber::Registry;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;

/// Track whether the global tracing subscriber has been installed.
///
/// - Guards `tracing::set_global_default`, which succeeds only once per process
/// - Keeps [`Logger::init`] idempotent across repeated [`ServiceProvider::init`] calls
static INIT: OnceLock<()> = OnceLock::new();

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
        let mut output: Result<(), Report<InitError>> = Ok(());
        INIT.get_or_init(|| {
            output = self.install();
        });
        output
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

    /// Install the global tracing subscriber.
    ///
    /// - Writes to `stderr` by default
    /// - Routes through `print!` machinery when the `testing` feature is enabled
    ///   so that `cargo test` can capture per-test output
    fn install(&self) -> Result<(), Report<InitError>> {
        let filter = self.build_targets();
        let layer = layer().compact();
        #[cfg(not(feature = "testing"))]
        let layer = layer.with_writer(stderr);
        #[cfg(feature = "testing")]
        let layer = layer.with_test_writer();
        let subscriber = layer
            .with_target(false)
            .with_timer(ElapsedTime::default())
            .with_filter(filter);
        set_global_default(Registry::default().with(subscriber)).change_context(InitError::Init)
    }
}
