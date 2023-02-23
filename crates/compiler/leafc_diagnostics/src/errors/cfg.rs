use miette::Diagnostic;
use owo_colors::OwoColorize;
use smol_str::SmolStr;
use strum_macros::EnumCount as EnumCountMacro;
use thiserror::Error;

/// Prefix appended to all `ConfigError` messages.
pub const CFG_ERROR_PREFIX: &str = "Config Error";

/// All possible errors that can occur during the compiler's
/// **configuration** process.
///
/// # Examples
///
/// ```rust
/// use leafc_diagnostics::errors::CfgError;
/// use strum::EnumCount;
///
/// // There are x variants of the `CfgError` enum.
/// assert_eq!(1, CfgError::COUNT);
/// ```
#[derive(Debug, Error, EnumCountMacro, Diagnostic)]
pub enum CfgError {
    /// This error is returned when **initializing** the compiler's
    /// configuration fails.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leafc_diagnostics::errors::CfgError;
    ///
    /// // The error is returned when an error occurs while initializing the compiler.
    /// let error = CfgError::Initialization("".into());
    /// ```
    #[error(
        "{} {} {}{} {}",
        CFG_ERROR_PREFIX.blue(),
        "-".black(),
        "Initialization failure: ".red(),
        ":".black(),
        .0.yellow().italic()
    )]
    #[diagnostic(
        code(leafc::cfg::initialization_failure),
        url(docsrs),
        help(
            "The current compiler configuration is invalid. This is likely a bug in the compiler. \
             Please report this issue to the developers."
        )
    )]
    Initialization(SmolStr),
}
