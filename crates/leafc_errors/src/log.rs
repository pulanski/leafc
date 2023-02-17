use miette::Diagnostic;
use owo_colors::OwoColorize;
use smol_str::SmolStr;
use strum_macros::EnumCount as EnumCountMacro;
use thiserror::Error;

/// Prefix appended to all `LexicalError` messages.
pub const LOG_ERROR_PREFIX: &str = "Logging System Error";

/// All possible errors that can occur during the **lexical analysis** phase of
/// the compiler.
///
/// # Examples
///
/// ```rust
/// use leafc_errors::log::LogError;
/// use strum::EnumCount;
///
/// // There are x variants of the `LexicalError` enum.
/// assert_eq!(3, LogError::COUNT);
/// ```
#[derive(Debug, Error, EnumCountMacro, Diagnostic, Clone)]
pub enum LogError {
    /// This error is returned when an **unknown token** is encountered
    /// during **lexical analysis**.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leafc_errors::log::LogError;
    ///
    /// // The error is returned when
    /// let error = LogError::LogFileInitialization("".into());
    /// ```
    #[error(
        "{} {} {}{} {}",
        LOG_ERROR_PREFIX.blue(),
        "-".black(),
        "Log file initialization failed".red(),
        ":".black(),
        .0.yellow().italic()
    )]
    #[diagnostic(
        code(leafc::log::log_file_initialization),
        url(docsrs),
        help(
            "The log file could not be initialized. Please try again (and report this issue if it \
             persists)."
        )
    )]
    LogFileInitialization(SmolStr),

    /// This error is returned when an **error occurs** during the
    /// **initialization** of the logging system.
    #[error(
        "{} {} {}{} {}",
        LOG_ERROR_PREFIX.blue(),
        "-".black(),
        "Log system initialization failed".red(),
        ":".black(),
        .0.yellow().italic()
    )]
    #[diagnostic(
        code(leafc::log::log_system_initialization),
        url(docsrs),
        help(
            "The log system could not be initialized. Please try again (and report this issue if \
             it persists)."
        )
    )]
    LogSystemInitialization(SmolStr),

    /// This error is returned when the **log file** could not be **opened**.
    /// This error is typically returned during the **initialization** of the
    /// logging system.
    #[error(
        "{} {} {}{} {}",
        LOG_ERROR_PREFIX.blue(),
        "-".black(),
        "Log file could not be opened".red(),
        ":".black(),
        .0.yellow().italic()
    )]
    #[diagnostic(
        code(leafc::log::log_file),
        url(docsrs),
        help(
            "The log file could not be opened. Please try again (and report this issue if it \
             persists)."
        )
    )]
    LogFileOpen(SmolStr),
}
