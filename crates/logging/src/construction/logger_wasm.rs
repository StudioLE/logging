use crate::prelude::*;
use tracing::subscriber::set_global_default;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{Layer, Registry};
use tracing_wasm::WASMLayer;

impl Logger {
    /// Set the global tracing subscriber.
    ///
    /// - Browser devtools render their own timestamps so [`ElapsedTime`] is not applied
    pub(super) fn set(&self) -> Result<(), Report<InitError>> {
        let filter = self.build_targets();
        let layer = WASMLayer::default().with_filter(filter);
        let registry = Registry::default().with(layer);
        set_global_default(registry).change_context(InitError::Init)
    }
}
