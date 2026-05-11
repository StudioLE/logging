//! Timer displaying elapsed seconds since subscriber initialization.
use crate::prelude::*;
use std::time::Instant;
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;

/// Timer displaying elapsed seconds since subscriber initialization.
pub struct ElapsedTime {
    start: Instant,
}

impl Default for ElapsedTime {
    fn default() -> Self {
        Self {
            start: Instant::now(),
        }
    }
}

impl FormatTime for ElapsedTime {
    fn format_time(&self, w: &mut Writer<'_>) -> FmtResult {
        let elapsed = self.start.elapsed();
        write!(w, "{:.3}", elapsed.as_secs_f64())
    }
}
