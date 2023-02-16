use miette::Diagnostic;
use owo_colors::OwoColorize;
use smol_str::SmolStr;
use strum_macros::EnumCount as EnumCountMacro;
use thiserror::Error;

/// Prefix appended to all `CliError` messages.
pub const CLI_ERROR_PREFIX: &str = "CLI Error";

/// All possible errors that can occur during the **lexical analysis** phase of
/// the compiler.
///
/// # Examples
///
/// ```rust
/// use leafc_errors::lexer::LexicalError;
/// use strum::EnumCount;
///
/// // There are x variants of the `LexicalError` enum.
/// assert_eq!(1, LexicalError::COUNT);
/// ```
#[derive(Debug, Error, EnumCountMacro, Diagnostic)]
pub enum CliError {
    /// This error is returned when an **unknown token** is encountered
    /// during **lexical analysis**.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leafc_errors::lexer::LexicalError;
    ///
    /// // The error is returned when an unknown token is encountered.
    /// let error = LexicalError::UnknownToken("".into());
    /// ```
    #[error(
        "{} {} {}{} {}",
        CLI_ERROR_PREFIX.blue(),
        "-".black(),
        "File not found: ".red(),
        ":".black(),
        .0.yellow().italic()
    )]
    #[diagnostic(
        code(leafc::cli::file_not_found),
        url(docsrs),
        help(
            "The file could not be found. Please check that the file exists and that you have \
             permission to access it."
        )
    )]
    FileNotFound(SmolStr),
}
