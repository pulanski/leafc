#![allow(unused_macros)]
use leafc_cfg::settings::LogLevel;

macro_rules! trace {
    ($topic:expr, $message:expr) => {
        if log::log_enabled!(log::Level::Trace) {
            log::log!(log::Level::Trace, "[{}]: {}", $topic, $message);
        }
    };
}

/// A macro that is used to log a **debug** message.
///
/// This is a **higher-level** of logging than [`trace!`], and is used for
/// annotating **behavior** that is **useful for debugging**.
macro_rules! debug {
    ($topic:expr, $message:expr) => {
        if log::log_enabled!(log::Level::Debug) {
            log::log!(log::Level::Debug, "[{}]: {}", $topic, $message);
        }
    };
}

macro_rules! info {
    ($topic:expr, $message:expr) => {
        if log::log_enabled!(log::Level::Info) {
            log::log!(log::Level::Info, "[{}]: {}", $topic, $message);
        }
    };
}

macro_rules! warn {
    ($topic:expr, $message:expr) => {
        if log::log_enabled!(log::Level::Warn) {
            log::log!(log::Level::Warn, "[{}]: {}", $topic, $message);
        }
    };
}

macro_rules! error {
    ($topic:expr, $message:expr) => {
        if log::log_enabled!(log::Level::Error) {
            log::log!(log::Level::Error, "[{}]: {}", $topic, $message);
        }
    };
}

macro_rules! fatal {
    ($topic:expr, $message:expr) => {
        if log::log_enabled!(log::Level::Error) {
            log::log!(log::Level::Error, "[{}]: {}", $topic, $message);
        }
    };
}

fn log_enabled(log_level: LogLevel) -> bool {
    match log_level {
        LogLevel::Trace => log::log_enabled!(log::Level::Trace),
        LogLevel::Debug => log::log_enabled!(log::Level::Debug),
        LogLevel::Info => log::log_enabled!(log::Level::Info),
        LogLevel::Warn => log::log_enabled!(log::Level::Warn),
        LogLevel::Error => log::log_enabled!(log::Level::Error),
        LogLevel::Fatal => log::log_enabled!(log::Level::Error),
    }
}

// macro_rules! log_enabled {
//     () => {

//     };
// }

#[cfg(test)]
mod tests {
    use leafc_cfg::settings::LogTopic;

    #[test]
    fn test_trace() {
        trace!(LogTopic::Lexer, "Hello, world!");
    }

    #[test]
    fn test_debug() {
        debug!(LogTopic::Lexer, "Hello, world!");
    }

    #[test]
    fn test_info() {
        info!(LogTopic::Lexer, "Hello, world!");
    }

    #[test]
    fn test_warn() {
        warn!(LogTopic::Lexer, "Hello, world!");
    }

    #[test]
    fn test_error() {
        error!(LogTopic::Lexer, "Hello, world!");
    }

    #[test]
    fn test_fatal() {
        fatal!(LogTopic::Lexer, "Hello, world!");
    }
}
