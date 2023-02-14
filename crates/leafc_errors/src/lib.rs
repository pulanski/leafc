pub mod cfg;
pub mod cli;
pub mod driver;
pub mod lexer;
pub mod log;
pub mod repl;
// pub mod parser;

pub use cfg::CfgError;
pub use cli::CliError;
pub use driver::DriverError;
pub use lexer::LexicalError;
pub use log::LogError;
pub use repl::ReplError;

/// The various **kinds of errors** that can occur within the compiler. These
/// include errors that occur while **parsing the command line arguments** ([`CliError`]),
/// that occur while **lexing the source code** ([`LexicalError`]), that occur
/// while **type-checking the source code**, and more.
///
/// The top-level [`LeafcError`] enum is used to represent all of the possible
/// errors that can occur within the compiler. This enum is then used to
/// implement the [`IntoDiagnostic`] trait for the [`LeafcError`] type, which
/// allows for the errors to be converted into a **human-readable diagnostic
/// message**.
///
/// // TODO: refactor the lexical error and parser error types to a single
/// // syntax error type (more relevant to the user).
///
/// ## Error variants include:
///
/// [`CliError`]
///
/// [`LexicalError`]
///
/// [`LogError`]
///
/// [`ParserError`]
///
/// [`TypeCheckError`]
///
/// [`CodegenError`]
///
/// [`ReplError`]
pub enum LeafcError {
    /// An error that occurred while **parsing the command line arguments**.
    CliError(CliError),
    /// An error that occurred while **running the driver**.
    DriverError(DriverError),
    /// An error that occurred while **lexing** the source code.
    LexicalError(LexicalError),
    /// An error that occurred within the **logging system**.
    LogError(LogError),
    // /// An error that occurred while parsing the source code.
    // ParserError(ParserError),
    // /// An error that occurred while type checking the source code.
    // TypeCheckError(TypeCheckError),
    // /// An error that occurred while generating the output.
    // CodegenError(CodegenError),
    /// An error that occurred while **running the repl**.
    ReplError(ReplError),
}
