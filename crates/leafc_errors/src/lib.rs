pub mod cfg;
pub mod cli;
pub mod driver;
pub mod lexer;
pub mod log;
pub mod repl;
pub mod syntax;
// pub mod parser;

use std::collections::VecDeque;

pub use cfg::CfgError;
pub use cli::CliError;
use derivative::Derivative;
pub use driver::DriverError;
use leafc_utils::Locatable;
pub use lexer::LexicalError;
pub use log::LogError;
pub use repl::ReplError;
pub use syntax::SyntaxError;

/// A **convenience type** for representing an **error** that is emitted by the
/// compiler. This type is used to **wrap** the [`LeafcError`] enum, which
/// represents all of the possible errors that can occur within the compiler.
/// Additionally, the type is used to **store** the **location** of the error
/// within the source code, which is used to **highlight** the error within the
/// source code when the error is emitted to the user.
///
/// # Examples
///
/// ```rust
/// use leafc_errors::Error;
/// ```
pub type Error = Locatable<LeafcError>;

/// A **convenience type** for representing a **warning** that is emitted by the
/// compiler. This type is used to **wrap** the [`LeafcWarning`] enum, which
/// represents all of the possible warnings that can occur within the compiler.
/// Additionally, the type is used to **store** the **location** of the warning
/// within the source code, which is used to **highlight** the warning within
/// the source code when the warning is emitted to the user.
///
/// # Examples
///
/// ```rust
/// use leafc_errors::Warning;
/// ```
pub type Warning = Locatable<LeafcWarning>;

/// A **diagnostic** is a message that is emitted by the compiler to inform the
/// user of an error or warning.
///
/// # Examples
///
/// ```rust
/// use leafc_errors::LeafcDiagnostic;
/// // TODO: add examples
/// ```
#[derive(Debug, Clone)]
pub enum LeafcDiagnostic {
    Error(Error),
    Warning(Warning),
}

/// The top-level **manager** for the **diagnostics** that are emitted by the
/// compiler. This manager is used to **store** the **errors** and **warnings**
/// that are emitted by the compiler.
///
/// # Examples
///
/// ```rust
/// use leafc_errors::DiagnosticsManager;
///
/// // TODO: add examples
/// ```
#[derive(Debug, Clone, Derivative)]
// #[derive(Debug, Default, Clone, Derivative, Serialize, Deserialize)]
// // TODO: add serde support
#[derivative(Default(new = "true"))]
pub struct DiagnosticsManager {
    errors:   VecDeque<Error>,
    warnings: VecDeque<Warning>,
}

/// The various **kinds of errors** that can occur within the compiler. These
/// include errors that occur while **parsing the command line arguments**
/// ([`CliError`]), that occur while **lexing the source code**
/// ([`LexicalError`]), that occur while **type-checking the source code**, and
/// more.
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
/// [`CliError`] - An error that occurred while **parsing the command line
/// arguments**.
///
/// [`LexicalError`] - An error that occurred while **lexing** the source code.
///
/// [`ParserError`]
///
/// [`SyntaxError`] - An error that occurred within the **syntax** of the source
/// code (i.e. lexical errors or parsing-related errors)
///
/// [`TypeCheckError`]
///
/// [`CodegenError`]
///
/// [`ReplError`] - An error that occurred while **running the repl**.
///
/// [`LogError`] - An error that occurred within the **logging system**.
#[derive(Debug, Clone)]
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
    /// An error that occurred within the **syntax** of the source code.
    SyntaxError(SyntaxError),
    /// An error that occurred while **running the repl**.
    ReplError(ReplError),
}

/// All possible **warnings** that can occur within the user's source code from
/// the perspective of the compiler.
///
/// # Examples
///
/// ```rust
/// use leafc_errors::LeafcWarning;
/// // TODO: add examples
/// ```
#[derive(Debug, Clone)]
pub enum LeafcWarning {
    UnusedImport,
    UnusedVariable,
    UnusedFunction,
    // Something like this..
    // UnusedType,
    // UnusedTrait,
    // UnusedImpl,
    // UnusedStruct,
    // UnusedEnum,
    // UnusedUnion,
    // TODO: maybe structure like this? need to figure out what possible warnings are
    // /// A warning that occurred while **parsing the command line arguments**.
    // CliWarning(CliError),
    // /// A warning that occurred while **running the driver**.
    // DriverWarning(DriverError),
    // /// A warning that occurred while **lexing** the source code.
    // LexicalWarning(LexicalError),
    // /// A warning that occurred within the **logging system**.
    // LogWarning(LogError),
    // // /// A warning that occurred while parsing the source code.
    // // ParserWarning(ParserError),
    // // /// A warning that occurred while type checking the source code.
    // // TypeCheckWarning(TypeCheckError),
    // // /// A warning that occurred while generating the output.
    // // CodegenWarning(CodegenError),
    // /// A warning that occurred within the **syntax** of the source code.
    // SyntaxWarning(SyntaxError),
    // /// A warning that occurred while **running the repl**.
    // ReplWarning(ReplError),
}

impl DiagnosticsManager {
    // /// Creates a new diagnostics manager.
    // pub fn new() -> Self {
    //     Self::default()
    // }

    /// Returns the **number of errors** that have been collected by the
    /// compiler during the **current compilation session**.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leafc_errors::DiagnosticsManager;
    /// // TODO: add examples
    /// ```
    pub fn num_errors(&self) -> usize {
        self.errors.len()
    }

    /// Returns the number of warnings that have been emitted by the compiler.
    pub fn num_warnings(&self) -> usize {
        self.warnings.len()
    }

    /// Returns `true` if the diagnostics manager has any errors.
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Returns `true` if the diagnostics manager has any warnings.
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// Returns `true` if the diagnostics manager has any errors or warnings.
    pub fn has_diagnostics(&self) -> bool {
        self.has_errors() || self.has_warnings()
    }

    /// Returns an iterator over the errors that have been emitted by the
    /// compiler.
    pub fn errors(&self) -> impl Iterator<Item = &Locatable<LeafcError>> {
        self.errors.iter()
    }

    /// Returns an iterator over the warnings that have been emitted by the
    /// compiler.
    pub fn warnings(&self) -> impl Iterator<Item = &Locatable<LeafcWarning>> {
        self.warnings.iter()
    }

    // /// Returns an iterator over the diagnostics that have been emitted by the
    // /// compiler.
    // pub fn diagnostics(&self) -> impl Iterator<Item =
    // &Locatable<LeafcDiagnostic>> {     self.errors.iter().map(|error|
    // error.map(|error| LeafcDiagnostic::Error(error))).chain(         self.
    // warnings             .iter()
    //             .map(|warning| warning.map(|warning|
    // LeafcDiagnostic::Warning(warning))),     )
    // }

    /// Emits an error to the diagnostics manager.
    pub fn emit_error(&mut self, error: Locatable<LeafcError>) {
        self.errors.push_back(error);
    }

    /// Emits a warning to the diagnostics manager.
    pub fn emit_warning(&mut self, warning: Locatable<LeafcWarning>) {
        self.warnings.push_back(warning);
    }

    // /// Emits a diagnostic to the diagnostics manager.
    // pub fn emit_diagnostic(&mut self, diagnostic:
}

#[cfg(test)]
mod error_test_suite {
    use super::*;
    use pretty_assertions_sorted::assert_eq;

    #[test]
    fn test_diagnostics_manager() {
        let mut diagnostics_manager = DiagnosticsManager::new();

        assert_eq!(diagnostics_manager.num_errors(), 0);
        assert_eq!(diagnostics_manager.num_warnings(), 0);
        assert_eq!(diagnostics_manager.has_errors(), false);
        assert_eq!(diagnostics_manager.has_warnings(), false);
        assert_eq!(diagnostics_manager.has_diagnostics(), false);

        // diagnostics_manager.emit_error(Locatable::new(
        //     LeafcError::CliError(CliError::InvalidOption),
        //     Location::new(0, 0),
        // ));

        // assert_eq!(diagnostics_manager.num_errors(), 1);
        // assert_eq!(diagnostics_manager.num_warnings(), 0);
        // assert_eq!(diagnostics_manager.has_errors(), true);
        // assert_eq!(diagnostics_manager.has_warnings(), false);
        // assert_eq!(diagnostics_manager.has_diagnostics(), true);

        // diagnostics_manager
        //     .emit_warning(Locatable::new(LeafcWarning::UnusedImport,
        // Location::new(0, 0)));

        // assert_eq!(diagnostics_manager.num_errors(), 1);
        // assert_eq!(diagnostics_manager.num_warnings(), 1);
        // assert_eq!(diagnostics_manager.has_errors(), true);
        // assert_eq!(diagnostics_manager.has_warnings(), true);
        // assert_eq!(diagnostics_manager.has_diagnostics(), true);
    }
}
