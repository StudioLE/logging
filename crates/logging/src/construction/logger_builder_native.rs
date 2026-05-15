//! Native-only builder methods for [`LoggerBuilder`].
use crate::prelude::*;
use tracing_subscriber::fmt::MakeWriter;

impl LoggerBuilder {
    /// Route formatted log output through the given [`MakeWriter`].
    ///
    /// - Replaces the default `stderr` (or [`tracing_subscriber::fmt::TestWriter`]
    ///   under the `testing` feature) sink
    /// - Use with `studiole_command::IndicatifWriter` to coordinate with progress bars
    #[must_use]
    pub fn with_writer<W>(mut self, writer: W) -> Self
    where
        W: for<'a> MakeWriter<'a> + Send + Sync + 'static,
    {
        self.writer = Some(SharedWriter::new(writer));
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::io;
    use tracing_subscriber::fmt::MakeWriter;

    struct MockWriter;

    impl<'a> MakeWriter<'a> for MockWriter {
        type Writer = io::Sink;
        fn make_writer(&'a self) -> Self::Writer {
            io::sink()
        }
    }

    /// [`LoggerBuilder::with_writer`] stores the writer in the built [`Logger`].
    #[test]
    fn logger_builder_with_writer() {
        let logger = LoggerBuilder::new().with_writer(MockWriter).build();
        assert!(logger.writer.is_some(), "writer should be stored");
    }
}
