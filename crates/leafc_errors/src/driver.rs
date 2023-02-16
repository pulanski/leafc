use miette::Diagnostic;
use owo_colors::OwoColorize;
use smol_str::SmolStr;
use strum_macros::EnumCount as EnumCountMacro;
use thiserror::Error;

/// Prefix appended to all `DriverError` messages.
pub const DRIVER_ERROR_PREFIX: &str = "Driver Error";

/// All possible errors that can occur during the execution of the compiler.
/// These errors are **not** related to the **source code** itself, but rather
/// errors that occur while **running the compiler**.
///
/// # Examples
///
/// ```rust
/// use leafc_errors::DriverError;
/// use strum::EnumCount;
///
/// // There are x variants of the `DriverError` enum.
/// assert_eq!(1, DriverError::COUNT);
/// ```
#[derive(Debug, Error, EnumCountMacro, Diagnostic)]
pub enum DriverError {
    /// This error is returned when an **unknown token** is encountered
    /// during **lexical analysis**.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leafc_errors::DriverError;
    ///
    /// // The error is returned when an error occurs while initializing the compiler.
    /// let error = DriverError::Initialization("".into());
    /// ```
    #[error(
        "{} {} {}{} {}",
        DRIVER_ERROR_PREFIX.blue(),
        "-".black(),
        "Initialization failure: ".red(),
        ":".black(),
        .0.yellow().italic()
    )]
    #[diagnostic(
        code(leafc::driver::initialization_failure),
        url(docsrs),
        help(
            "The driver failed to initialize. This is likely a bug in the compiler. Please report \
             this issue to the developers."
        )
    )]
    Initialization(SmolStr),
}
