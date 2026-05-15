//! Native platform tracing subscriber.
use crate::prelude::*;
#[cfg(not(feature = "testing"))]
use std::io::stderr;
use tracing::Subscriber;
use tracing::subscriber::set_global_default;
use tracing_subscriber::filter::Targets;
#[cfg(feature = "testing")]
use tracing_subscriber::fmt::TestWriter;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::fmt::writer::BoxMakeWriter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{Layer, Registry};

impl Logger {
    /// Set the global tracing subscriber.
    ///
    /// - Writes compact, [`ElapsedTime`]-stamped lines to `stderr` by default
    /// - With the `testing` feature, defaults to [`TestWriter`] so `cargo test`
    ///   captures per-test output
    /// - A writer supplied via [`LoggerBuilder::with_writer`] overrides both defaults
    pub(super) fn set(&self) -> Result<(), Report<InitError>> {
        let filter = self.build_targets();
        let registry = build_registry(filter, self.writer.clone());
        set_global_default(registry).change_context(InitError::Init)
    }
}

/// Build the configured subscriber registry without installing it.
///
/// - Extracted from [`Logger::set`] so tests can attach the subscriber via
///   [`tracing::subscriber::with_default`] instead of [`set_global_default`]
fn build_registry(filter: Targets, writer: Option<SharedWriter>) -> impl Subscriber {
    let make_writer = writer.unwrap_or_else(|| SharedWriter::from_box(default_make_writer()));
    let layer = layer()
        .compact()
        .with_writer(make_writer)
        .with_target(false)
        .with_timer(ElapsedTime::default())
        .with_filter(filter);
    Registry::default().with(layer)
}

/// Default writer factory used when no custom writer is supplied.
#[cfg(not(feature = "testing"))]
fn default_make_writer() -> BoxMakeWriter {
    BoxMakeWriter::new(stderr)
}

/// Testing-feature default: route through [`TestWriter`] so `cargo test` captures output.
#[cfg(feature = "testing")]
fn default_make_writer() -> BoxMakeWriter {
    BoxMakeWriter::new(TestWriter::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Write};
    use std::sync::{Arc, Mutex, MutexGuard};
    use tracing::subscriber::with_default;
    use tracing_subscriber::filter::LevelFilter;
    use tracing_subscriber::fmt::MakeWriter;

    #[derive(Clone)]
    struct BufferWriter(Arc<Mutex<Vec<u8>>>);

    struct BufferGuard<'a>(MutexGuard<'a, Vec<u8>>);

    impl Write for BufferGuard<'_> {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.0.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    impl<'a> MakeWriter<'a> for BufferWriter {
        type Writer = BufferGuard<'a>;
        fn make_writer(&'a self) -> Self::Writer {
            BufferGuard(self.0.lock().expect("buffer lock"))
        }
    }

    /// [`Logger::set`] honors the custom writer set via [`LoggerBuilder::with_writer`].
    #[test]
    fn logger_set_routes_through_custom_writer() {
        // Arrange
        let buffer = Arc::new(Mutex::new(Vec::<u8>::new()));
        let writer = BufferWriter(buffer.clone());
        let filter = Targets::new().with_default(LevelFilter::from(LogLevel::Info));
        let registry = build_registry(filter, Some(SharedWriter::new(writer)));
        // Act
        with_default(registry, || {
            tracing::info!("hello world");
        });
        // Assert
        let captured =
            String::from_utf8(buffer.lock().expect("buffer lock").clone()).expect("utf-8");
        assert!(
            captured.contains("hello world"),
            "captured output should contain message, got: {captured}"
        );
    }
}
