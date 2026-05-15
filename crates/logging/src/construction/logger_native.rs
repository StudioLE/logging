use crate::prelude::*;
#[cfg(not(feature = "testing"))]
use std::io::stderr;
use tracing::subscriber::set_global_default;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{Layer, Registry};

impl Logger {
    /// Set the global tracing subscriber.
    ///
    /// - Writes compact, [`ElapsedTime`]-stamped lines to `stderr`
    /// - With the `testing` feature, routes through `print!` so `cargo test` captures per-test output
    pub(super) fn set(&self) -> Result<(), Report<InitError>> {
        let filter = self.build_targets();
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
}
