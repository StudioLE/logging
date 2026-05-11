//! Log level configuration.

use crate::prelude::*;
use tracing_subscriber::filter::LevelFilter;

/// Verbosity of log output.
#[derive(Clone, Copy, Debug, Deserialize, Default, Eq, PartialEq, Serialize)]
pub enum LogLevel {
    /// Errors only.
    Error,
    /// Errors and warnings.
    Warn,
    /// Errors, warnings, and info.
    #[default]
    Info,
    /// Errors, warnings, info, and debug.
    Debug,
    /// Errors, warnings, info, debug, and trace.
    Trace,
}

impl LogLevel {
    /// Get the log level as a title case string.
    pub fn as_str(self) -> &'static str {
        match self {
            LogLevel::Error => "Error",
            LogLevel::Warn => "Warn",
            LogLevel::Info => "Info",
            LogLevel::Debug => "Debug",
            LogLevel::Trace => "Trace",
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(self.as_str())
    }
}

impl FromStr for LogLevel {
    type Err = ParseLogLevelError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "error" => Ok(Self::Error),
            "warn" => Ok(Self::Warn),
            "info" => Ok(Self::Info),
            "debug" => Ok(Self::Debug),
            "trace" => Ok(Self::Trace),
            _ => Err(ParseLogLevelError),
        }
    }
}

impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Error => Self::ERROR,
            LogLevel::Warn => Self::WARN,
            LogLevel::Info => Self::INFO,
            LogLevel::Debug => Self::DEBUG,
            LogLevel::Trace => Self::TRACE,
        }
    }
}

/// Returned when a string cannot be parsed into a [`LogLevel`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("invalid log level, expected: error, warn, info, debug, trace")]
pub struct ParseLogLevelError;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn log_level_default() {
        let level = LogLevel::default();
        assert_eq!(level, LogLevel::Info);
    }

    #[test]
    fn log_level_from_str() {
        assert_eq!(LogLevel::from_str("info"), Ok(LogLevel::Info));
        assert_eq!(LogLevel::from_str("DEBUG"), Ok(LogLevel::Debug));
        assert_eq!(LogLevel::from_str("Warn"), Ok(LogLevel::Warn));
    }

    #[test]
    fn log_level_from_str_invalid() {
        assert_eq!(LogLevel::from_str("verbose"), Err(ParseLogLevelError));
    }
}
