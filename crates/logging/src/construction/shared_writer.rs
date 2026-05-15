//! Cheaply-cloneable writer wrapper for the tracing subscriber.
use crate::prelude::*;
use std::sync::Arc;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::fmt::writer::BoxMakeWriter;

/// Cheaply-cloneable [`MakeWriter`] wrapper around [`BoxMakeWriter`].
///
/// - [`BoxMakeWriter`] is not [`Clone`], but `Logger::init` receives `&self`
///   and must clone the stored writer to hand it to `Logger::set`
/// - Internal type; [`LoggerBuilder::with_writer`] is the public entry point
#[derive(Clone)]
pub(crate) struct SharedWriter(Arc<BoxMakeWriter>);

impl SharedWriter {
    /// Create a new [`SharedWriter`] from any [`MakeWriter`].
    pub(crate) fn new<W>(writer: W) -> Self
    where
        W: for<'a> MakeWriter<'a> + Send + Sync + 'static,
    {
        Self::from_box(BoxMakeWriter::new(writer))
    }

    /// Create a new [`SharedWriter`] from an existing [`BoxMakeWriter`].
    ///
    /// - Avoids double-wrapping when the input is already a [`BoxMakeWriter`]
    pub(crate) fn from_box(writer: BoxMakeWriter) -> Self {
        Self(Arc::new(writer))
    }
}

impl Debug for SharedWriter {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str("SharedWriter(..)")
    }
}

impl<'a> MakeWriter<'a> for SharedWriter {
    type Writer = <BoxMakeWriter as MakeWriter<'a>>::Writer;
    fn make_writer(&'a self) -> Self::Writer {
        self.0.make_writer()
    }
}
