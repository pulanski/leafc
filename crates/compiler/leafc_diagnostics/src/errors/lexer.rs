use miette::Diagnostic;
use owo_colors::OwoColorize;
use smol_str::SmolStr;
use strum_macros::EnumCount as EnumCountMacro;
use thiserror::Error;

/// Prefix appended to all `LexicalError` messages.
pub const LEXICAL_ERROR_PREFIX: &str = "Lexical Error";

/// All possible errors that can occur during the **lexical analysis** phase of
/// the compiler.
///
/// # Examples
///
/// ```rust
/// use leafc_diagnostics::errors::lexer::LexicalError;
/// use strum::EnumCount;
///
/// // There are x variants of the `LexicalError` enum.
/// assert_eq!(1, LexicalError::COUNT);
/// ```
#[derive(Debug, Error, EnumCountMacro, Diagnostic, Clone)]
pub enum LexicalError {
    /// This error is returned when an **unknown token** is encountered
    /// during **lexical analysis**.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leafc_diagnostics::errors::lexer::LexicalError;
    ///
    /// // The error is returned when an unknown token is encountered.
    /// let error = LexicalError::UnknownToken("".into());
    /// ```
    #[error(
        "{} {} {}{} {}",
        LEXICAL_ERROR_PREFIX.blue(),
        "-".black(),
        "Unknown token".red(),
        ":".black(),
        .0.yellow().italic()
    )]
    #[diagnostic(
        code(leafc::lexer::unknown_token),
        url(docsrs),
        help(
            "The token you've entered is not valid (contained within the grammar of the language)."
        )
    )]
    UnknownToken(SmolStr),
}
