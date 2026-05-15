//! Logger service that initializes the global tracing subscriber.
use crate::prelude::*;
use std::sync::OnceLock;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::filter::Targets;

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
    /// Optional custom writer for formatted log output.
    ///
    /// - When [`Some`], replaces the default `stderr`/`TestWriter` sink
    /// - Set via [`LoggerBuilder::with_writer`]
    /// - Default: [`None`]
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) writer: Option<SharedWriter>,
}

impl Init for Logger {
    fn init(&self, _services: &ServiceProvider) -> Result<(), Report<InitError>> {
        let mut output: Result<(), Report<InitError>> = Ok(());
        INIT.get_or_init(|| {
            output = self.set();
        });
        output
    }
}

impl Logger {
    /// Build the [`Targets`] filter from level and per-target overrides.
    pub(super) fn build_targets(&self) -> Targets {
        let mut targets = Targets::new().with_default(LevelFilter::from(self.level));
        for filter in &self.targets {
            targets = targets.with_target(&filter.name, LevelFilter::from(filter.level));
        }
        targets
    }
}
