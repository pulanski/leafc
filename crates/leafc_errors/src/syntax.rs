use codespan_reporting::{
    diagnostic::{
        Diagnostic,
        Label,
    },
    files::Files as SourceFiles,
    term::{
        self,
        termcolor::StandardStream,
        Config,
    },
};
// use miette::Diagnostic;
use owo_colors::OwoColorize;
use smol_str::SmolStr;
use strum_macros::EnumCount as EnumCountMacro;
use thiserror::Error;

// use crate::LexicalError;

/// Prefix appended to all `SyntaxError` messages.
pub const SYNTAX_ERROR_PREFIX: &str = "Syntax Error";

/// All possible **syntax errors** that can occur within the user's source code
/// from the perspective of the compiler.
///
/// # Examples
///
/// ```rust
/// use leafc_errors::syntax::SyntaxError;
/// use strum::EnumCount;
///
/// // There are x variants of the `LexicalError` enum.
/// assert_eq!(1, SyntaxError::COUNT);
/// ```
#[derive(Debug, Error, EnumCountMacro, Clone)]
pub enum SyntaxError {
    /// This error is returned when an **unknown token** is encountered
    /// during **lexical analysis**.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leafc_errors::syntax::SyntaxError;
    ///
    /// // The error is returned when an unknown token is encountered.
    /// let error = SyntaxError::UnknownToken("".into());
    /// ```
    #[error(
        "{} {} {}{} {}",
        SYNTAX_ERROR_PREFIX.blue(),
        "-".black(),
        "Unknown token".red(),
        ":".black(),
        .0.yellow().italic()
    )]
    UnknownToken(SmolStr),
}

fn foo() {
    println!("Hello, world!");
}

// impl From<LexicalError> for SyntaxError {
//     fn from(error: LexicalError) -> Self {
//         match error {
//             LexicalError::UnknownToken(token) => Self::UnknownToken(token),
//         }
//     }
// }

// /// This error is returned when an **unknown token** is encountered
// /// during **lexical analysis**.
// ///
// /// # Examples
// ///
// /// ```rust
// /// use leafc_errors::syntax::SyntaxError;
// ///
// /// // The error is returned when an unknown token is encountered.
// /// let error = SyntaxError::UnknownToken("".into());
// /// ```
// #[error(
//     "{} {} {}{} {}",
//     SYNTAX_ERROR_PREFIX.blue(),
//     "-".black(),
//     "Unknown token".red(),
//     ":".black(),
//     .0.yellow().italic()
// )]
// #[diagnostic(
//     code(leafc::syntax::unknown_token),
//     url(docsrs),
//     help(
//         "The token you've entered is not valid (contained within the grammar
// of the language)."     )
// )]
// UnknownToken(SmolStr),
