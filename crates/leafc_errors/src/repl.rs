use miette::Diagnostic;
use owo_colors::OwoColorize;
use smol_str::SmolStr;
use strum_macros::EnumCount as EnumCountMacro;
use thiserror::Error;

/// Prefix appended to all `ReplError` messages.
pub const REPL_ERROR_PREFIX: &str = "REPL Error";

/// All possible errors that can occur during the **execution** of the **REPL**.
///
/// # Examples
///
/// ```rust
/// use leafc_errors::repl::ReplError;
/// use strum::EnumCount;
///
/// // There are x variants of the `ReplError` enum.
/// assert_eq!(3, ReplError::COUNT);
/// ```
#[derive(Debug, Error, EnumCountMacro, Diagnostic)]
pub enum ReplError {
    /// This error is returned when the **log file** could not be **opened**.
    /// This error is typically returned during the **initialization** of the
    /// REPL.
    #[error(
        "{} {} {}{} {}",
        REPL_ERROR_PREFIX.blue(),
        "-".black(),
        "Log file could not be opened".red(),
        ":".black(),
        .0.yellow().italic()
    )]
    #[diagnostic(
        code(leafc::repl::log_file_open),
        url(docsrs),
        help("The log file could not be opened. Please try again (and report this issue if it persists).")
    )]
    LogFileOpen(SmolStr),

    /// This error is returned when the **history file** could not be **opened**.
    /// This error is typically returned during the **initialization** of the
    /// REPL.
    #[error(
        "{} {} {}{} {}",
        REPL_ERROR_PREFIX.blue(),
        "-".black(),
        "History file could not be opened".red(),
        ":".black(),
        .0.yellow().italic()
    )]
    #[diagnostic(
        code(leafc::repl::history_file_open),
        url(docsrs),
        help("The history file could not be opened. Please try again (and report this issue if it persists).")
    )]
    HistoryFileOpen(SmolStr),

    /// This error is returned when an **invalid update** was attempted on the
    /// REPL's **settings**.
    /// This error is typically returned during the **execution** of the REPL.
    #[error(
        "{} {} {}{} {}",
        REPL_ERROR_PREFIX.blue(),
        "-".black(),
        "Invalid update to REPL settings".red(),
        ":".black(),
        .0.yellow().italic()
    )]
    #[diagnostic(
        code(leafc::repl::invalid_settings_update),
        url(docsrs),
        help("The REPL settings could not be updated. Please try again (and report this issue if it persists).")
    )]
    InvalidSettingsUpdate(SmolStr),
}
