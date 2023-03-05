pub mod cfg;
pub mod cli;
pub mod driver;
pub mod lexer;
pub mod log;
pub mod repl;
pub mod syntax;
// pub mod parser;

use codespan_reporting::diagnostic::Diagnostic;
use codespan_reporting::files::Files as SourceFiles;
use derivative::Derivative;
use getset::{
    Getters,
    MutGetters,
    Setters,
};
use std::{
    collections::VecDeque,
    ops::Range,
};

use leafc_utils::{
    codemap::TextPosition,
    FileId,
    Location,
    Span,
};

pub use cfg::CfgError;
pub use cli::CliError;
pub use driver::DriverError;
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
/// use leafc_diagnostics::errors::Error;
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
/// use leafc_diagnostics::errors::Warning;
/// ```
pub type Warning = Locatable<LeafcWarning>;

/// A **wrapper** around an **item** that also stores its **location**.
/// An **item** can be a **token**, a **node**, or a **span**.
///
/// # Example
///
/// ```ignore
/// // TODO: add example
/// use leafc_lexer::TokenStream;
/// use leafc_utils::Locatable;
/// use leafc_lexer::Token;
/// ```
#[derive(Debug, Clone, PartialEq, Getters, MutGetters, Setters)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct Locatable<T: Clone> {
    /// The **location** of the item.
    location: Location,
    /// The **item** itself.
    item:     T,
}

// TODO: refactor
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
    /// An error that occurred within the **source code**. These errors are
    /// emitted to the user via the [`codespan`] crate.
    Locatable(LocatableError),
    /// An error not associated with a specific location within the source code
    /// (i.e. an error that occurred while **parsing the command line
    /// arguments** or an error that occurred while **running the repl**). These
    /// errors are emitted to the user via the [`miette`] crate.
    NonLocatable(NonLocatableError),
}

/// All possible **errors** that can occur within the user's source code from
/// the perspective of the compiler (i.e. errors that occur while **parsing**,
/// **type-checking**, etc.).
///
/// These errors use the [`codespan`] crate to **highlight** the error within
/// the source code when the error is emitted to the user. An example of an
/// error that uses the [`codespan`] crate is the following:
///
/// ```
/// error[E0432]: unresolved import `token::TokenKind`
///   --> crates/compiler/leafc_lexer/src/lib.rs:30:5
///    |
/// 30 |     token::TokenKind,
///    |     ^^^^^^^^^^^^^^^^ no `TokenKind` in `token`
/// ```
#[derive(Debug, Clone)]
pub enum LocatableError {
    /// An error that occurred while **lexing** the source code.
    LexicalError(Locatable<LexicalError>), // Codespan
    // TypeCheckError(Locatable<TypeCheckError>),
    SyntaxError(Locatable<SyntaxError>), /* Codespan */

                                         /* An error that occurred while parsing the source
                                          * code.
                                          * ParserError(Locatable<ParserError>), // Codespan
                                          * /// An error that occurred while type checking the source code.
                                          * TypeCheckError(TypeCheckError), // Codespan */
}

/// All possible **errors** which do not have a **location** within the source
/// code. These errors are emitted to the user via the [`miette`] crate.
///
/// # Examples
///
/// ```text
/// Error: leafc::cli::file_not_found (https://docs.rs/leafc_diagnostics/0.1.0/leafc_diagnostics/enum.CliError.html#variant.FileNotFound)
///
///   × CLI Error - File not found: : "foo.leaf": No such file or directory (os
/// error 2)
///  help: The file could not be found. Please check that the file
/// exists and that you have permission to access it.
/// ```
#[derive(Debug, Clone)]
pub enum NonLocatableError {
    /// An error that occurred while **parsing the command line arguments**.
    CliError(CliError), // Miette
    /// An error that occurred while **running the driver**.
    DriverError(DriverError), // Miette
    /// An error that occurred within the **logging system**.
    LogError(LogError), /* Miette
                         * /// An error that occurred while generating the output.
                         * CodegenError(CodegenError), // Miette */
    /// An error that occurred while **running the repl**.
    ReplError(ReplError), // Miette
}

// pub enum LocatableWarning {
//     // TypeCheckError(Locatable<TypeCheckError>),
//     SyntaxWarning(Locatable<SyntaxWarning>),
// }

/// All possible **warnings** that can occur within the user's source code from
/// the perspective of the compiler.
///
/// # Examples
///
/// ```rust
/// use leafc_diagnostics::errors::LeafcWarning;
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

/// A **diagnostic** is a message that is emitted by the compiler to inform the
/// user of an error or warning.
///
/// # Examples
///
/// ```rust
/// use leafc_diagnostics::errors::LeafcDiagnostic;
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
/// use leafc_diagnostics::errors::DiagnosticsManager;
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

impl<T: Clone> Locatable<T> {
    /// Creates a new `Locatable` from the given `location` and `item`.
    ///
    /// # Example
    ///
    /// ```
    /// // TODO: add example
    /// use leafc_diagnostics::errors::Locatable;
    /// use leafc_utils::{
    ///     FileId,
    ///     Location,
    ///     Span,
    /// };
    ///
    /// let location = Location::new(FileId::new(0), Span::new(0, 1));
    /// ```
    pub const fn new(location: Location, item: T) -> Self {
        Self { location, item }
    }

    /// Returns the **span** of the `Locatable` item.
    ///
    /// # Example
    ///
    /// ```
    /// // TODO: add example
    /// use leafc_diagnostics::errors::Locatable;
    /// ```
    pub fn span(&self) -> Span {
        self.location.span()
    }

    /// Returns the **file** (i.e. the `FileId`) of the `Locatable` item.
    ///
    /// # Example
    ///
    /// ```
    /// // TODO: add example
    /// use leafc_diagnostics::errors::Locatable;
    /// ```
    pub fn file(&self) -> FileId {
        self.location.file_id()
    }

    /// Returns the **range** of the `Locatable` item.
    /// The **range** is the **span** of the `Locatable` item, more specifically
    /// a **tuple** of the **start** and **end** of the `Locatable` item.
    ///
    /// # Example
    ///
    /// ```
    /// // TODO: add example
    /// use leafc_diagnostics::errors::Locatable;
    /// ```
    pub fn range(&self) -> Range<TextPosition> {
        self.location.span().range()
    }

    /// Returns the **start** of the `Locatable` item.
    ///
    /// # Example
    ///
    /// ```
    /// // TODO: add example
    /// use leafc_diagnostics::errors::Locatable;
    /// ```
    pub fn start(&self) -> TextPosition {
        self.location.span().start()
    }

    /// Returns the **end** of the `Locatable` item.
    ///
    /// # Example
    ///
    /// ```
    /// // TODO: add example
    /// use leafc_diagnostics::errors::Locatable;
    /// ```
    pub fn end(&self) -> TextPosition {
        self.location.span().end()
    }

    /// Maps the `Locatable` item to another item using the given `f` function.
    ///
    /// # Example
    ///
    /// ```
    /// // TODO: add example
    /// use leafc_diagnostics::errors::Locatable;
    /// ```
    pub fn map<U: Clone, F: FnOnce(T) -> U>(self, f: F) -> Locatable<U> {
        Locatable::new(self.location, f(self.item))
    }
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
    /// use leafc_diagnostics::errors::DiagnosticsManager;
    /// // TODO: add examples
    /// ```
    pub fn num_errors(&self) -> usize {
        self.errors.len()
    }

    /// Returns the number of warnings that have been emitted by the compiler.
    pub fn num_warnings(&self) -> usize {
        self.warnings.len()
    }

    /// Returns `true` if the diagnostics manager has **any** errors.
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Returns `true` if the diagnostics manager has **any** warnings.
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// Returns `true` if the diagnostics manager has collected **any** errors
    /// or warnings.
    pub fn has_diagnostics(&self) -> bool {
        self.has_errors() || self.has_warnings()
    }

    /// Returns an **iterator** over the errors that have been collected by the
    /// compiler during the **current compilation session**.
    pub fn errors(&self) -> impl Iterator<Item = &Locatable<LeafcError>> {
        self.errors.iter()
    }

    /// Returns an **iterator** over the warnings that have been collected by
    /// the compiler during the **current compilation session**.
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

    /// **Adds** an error to the diagnostics manager.
    pub fn add_error(&mut self, error: Error) {
        self.errors.push_back(error);
    }

    /// **Adds** a warning to the diagnostics manager.
    pub fn add_warning(&mut self, warning: Warning) {
        self.warnings.push_back(warning);
    }

    /// **Adds** a diagnostic to the diagnostics manager. This method is a
    /// **convenience method** that **calls** the appropriate method based on
    /// the type of diagnostic that is being added (i.e. `add_error` or
    /// `add_warning`)
    pub fn add_diagnostic(&mut self, diagnostic: LeafcDiagnostic) {
        match diagnostic {
            LeafcDiagnostic::Error(error) => self.add_error(error),
            LeafcDiagnostic::Warning(warning) => self.add_warning(warning),
        }
    }

    /// **Adds** a collection of diagnostics to the diagnostics manager.
    /// This method is a **convenience method** that **calls** the
    /// `add_diagnostic` method for each diagnostic in the collection.
    pub fn add_diagnostics(&mut self, diagnostics: impl IntoIterator<Item = LeafcDiagnostic>) {
        for diagnostic in diagnostics {
            self.add_diagnostic(diagnostic);
        }
    }

    // pub fn emit<'a, F>(&mut self, files: &'a F, writer: &StandardStream, config:
    // &Config) where
    //     F: CodeFiles<'a, FileId = FileId>,
}

// error types, locatable error vs non-locatable error

impl LeafcError {
    // pub fn emit<'a, F>(
    //     &self,
    //     files: &'a F,
    //     file: FileId,
    //     span: Span,
    //     errs: &mut Vec<Diagnostic<FileId>>,
    // ) where
    //     F: SourceFiles<'a, FileId = FileId>,
    // {
    //     match self {
    //         // miette style errors should not be emitted (may change in the
    // future)         LeafcError::CliError(_) |
    //         LeafcError::DriverError(_) |
    //         LeafcError::LogError(_) |
    //         LeafcError::ReplError(_) => {
    //             unimplemented!()
    //         }
    //         LeafcError::LexicalError(_) => todo!(),
    //         LeafcError::SyntaxError(syntax_err) => syntax_err.emit(files, file,
    // span, errs),     }
    // }
}

// TODO: need to distinguish between miette-type errors and
// codespan-reporting-type errors

#[cfg(test)]
mod error_test_suite {
    use super::*;
    use pretty_assertions_sorted::assert_eq;

    #[test]
    fn test_diagnostics_manager() {
        let diagnostics_manager = DiagnosticsManager::new();

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
