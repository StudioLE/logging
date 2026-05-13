//! Logger service that initializes the global tracing subscriber.
use crate::prelude::*;
#[cfg(all(not(target_arch = "wasm32"), not(feature = "testing")))]
use std::io::stderr;
use std::sync::OnceLock;
use tracing::subscriber::set_global_default;
use tracing_subscriber::Layer;
use tracing_subscriber::Registry;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::filter::Targets;
#[cfg(not(target_arch = "wasm32"))]
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
#[cfg(target_arch = "wasm32")]
use tracing_wasm::WASMLayer;

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
    /// - Dispatches to [`install_wasm`] on `wasm32` targets
    /// - Dispatches to [`install_native`] on every other target
    fn install(&self) -> Result<(), Report<InitError>> {
        let filter = self.build_targets();
        #[cfg(target_arch = "wasm32")]
        return install_wasm(filter);
        #[cfg(not(target_arch = "wasm32"))]
        install_native(filter)
    }
}

/// Install a browser-console subscriber backed by [`tracing_wasm::WASMLayer`].
///
/// - Browser devtools render their own timestamps so [`ElapsedTime`] is not applied
#[cfg(target_arch = "wasm32")]
fn install_wasm(filter: Targets) -> Result<(), Report<InitError>> {
    let layer = WASMLayer::default().with_filter(filter);
    let registry = Registry::default().with(layer);
    set_global_default(registry).change_context(InitError::Init)
}

/// Install a formatted subscriber backed by [`tracing_subscriber::fmt`].
///
/// - Writes compact, elapsed-time-stamped lines to `stderr`
/// - With the `testing` feature, routes through `print!` instead so `cargo test`
///   captures per-test output
#[cfg(not(target_arch = "wasm32"))]
fn install_native(filter: Targets) -> Result<(), Report<InitError>> {
    let layer = layer().compact();
    #[cfg(not(feature = "testing"))]
    let layer = layer.with_writer(stderr);
    #[cfg(feature = "testing")]
    let layer = layer.with_test_writer();
    let layer = layer
        .with_target(false)
        .with_timer(ElapsedTime::default())
        .with_filter(filter);
    set_global_default(Registry::default().with(layer)).change_context(InitError::Init)
}
