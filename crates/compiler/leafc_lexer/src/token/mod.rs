#![doc = TOKEN_README!()]

use std::fmt;

use leafc_data_structures::collections::HashSet;

use derive_new::new;
use getset::{
    Getters,
    MutGetters,
    Setters,
};
use leafc_utils::Span;
use owo_colors::OwoColorize;
use smol_str::SmolStr;
use typed_builder::TypedBuilder;

#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

#[cfg(feature = "multi-threaded")]
use std::sync::Arc;

mod group;
#[macro_use]
mod macros;

pub mod kinds;

pub use {
    group::TokenGroup,
    kinds::TokenKind,
};

/// ## [**`Token`**][Token]
///
/// A **token** is a **lexical unit** of the source code. It is a **minimal
/// unit** of the language that has **meaning**. Tokens are **generated** by
/// the **lexer** and are **consumed** by the **parser**.
///
/// Tokens are **immutable** and **non-owning**. They are **copied** by the
/// parser and **moved** by the compiler.
///
/// # Example:
///
/// ```rust
/// //      use std::fmt;
/// //       ^  ^  ^ ^  ^
/// //       |  |  | |  |
/// //       |  |  | |  +-> `;`   is a  `SEMICOLON` token
/// //       |  |  | +----> `fmt` is an `IDENTIFIER` token
/// //       |  |  +------> `::`  is a  `PATH` token
/// //       |  +---------> `std` is an `IDENTIFIER` token
/// //       +------------> `use` is a  `USE_KW` token
/// ```
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    new,
    Getters,
    MutGetters,
    Setters,
    TypedBuilder,
)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct Token {
    /// The **kind** of the token (i.e. `WHITESPACE`, `IDENTIFIER`, etc.)
    kind:   TokenKind,
    /// The **lexeme** of the token (i.e. the **text** that the token
    /// represents)
    lexeme: SmolStr,
    // lexeme: StringId, TODO: use interned strings
    /// The **span** of the token (i.e. the **location** of the token in the
    /// input string)
    span:   Span,
}

/// ## [**`LanguageChecker`**][LanguageChecker]
///
/// A **language checker** is a **container** for a **spoken language** and all
/// of its **legal tokens**.
///
/// The **job** of the **language checker** is to **check** the **tokens** of a
/// **language** and **report** any **errors**, that is, **invalid** tokens. A
/// token is deemed **invalid** if it is **not** a **valid** token of the given
/// spoken language. The **language checker** is **used** during the phase of
/// **lexical analysis** to **check** the **tokens** of the **input string**.
/// // TODO: in fhe future, make the checker resilient to errors
/// // In general, when an invalid token is found, the checker should
/// // emit the error to a diagnostic sink and continue checking the
/// // rest of the tokens.
///
/// # Example:
///
/// ```rust
/// 
/// // TODO: refactor example to use rayon and interned strings, and
///
/// // new calls builder internally
/// use leafc_lexer::token::LanguageChecker;
///
/// let lang = LanguageChecker::builder()
///     .lang("en".into())
///     .tokens(vec!["false".into(), "true".into(), "Faux".into(), "falsch".into()])
///     .build();
///
/// assert_eq!(lang.lang(), "en");
///
/// // TODO: use interned strings (e.g. lang() -> str based on interned id)
pub struct LanguageChecker {
    pub lang:   SmolStr,
    // pub lang:   StringId, TODO: use interned strings
    pub tokens: TokenSet,
    // pub tokens: TokenSet,
}

impl LanguageChecker {}

/// ## [**`Checker`**][Checker]
///
/// A **checker** is a **trait** is used within some **context** of the
/// compiler to check the **validity** of source code. Examples of subsystems
/// that use checkers are the **lexer** and the **parser**. Combined together,
/// there is a more general **check pipeline** that is used to **check** the
/// **validity** of the **source code** (i.e. a **proof of correctness** that
/// the given **source code** will **compile**).
///
/// The **check pipeline** is **composed** of a series of **passes** that
/// **check** the **source code** for **errors** at various levels of **semantic
/// meaning** and associated internal **representation**. The **check pipeline**
/// is **used** to **check** the **source code** before it is **compiled** (i.e.
/// before **codegen** occurs).
///
/// This is effectively the **heart** of the **analysis** phase of the
/// **compiler**. Furthermore, this is the ideal place to **emit** any
/// **diagnostics** that are **generated** during the **analysis** phase, and
/// , in general, is the preferred way to **develop** the **compiler**, as it
/// allows for incremental watchmode compilation times (_rust build times are
/// notoriously slow_).
///
/// // TODO: in the core database crate, module name should be `check` within
/// an `analysis` module
trait Checker {
    // fn check(&self, source_file: &SourceFile) -> Result<LeafcDiagnostics>;

    fn check(&self, token: &Token) -> bool;
    // fn check(&self, token: &Token) -> (bool, LeafcDiagnostics); // TODO
}

#[derive(Debug, Clone)]
// #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "allocative", derive(Allocative))]
pub struct TokenSet {
    pub tokens: HashSet<Token>,
}

#[cfg(feature = "multi-threaded")]
pub struct LanguageTokens {
    pub tokens: Arc<HashSet<Token>>,
}

// pub struct LanguageTokens {
//     pub tokens: Vec<Token>,
// }

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}{}{} {}",
            self.kind.cyan(),
            "@".black().italic(),
            "[".red().bold(),
            self.span,
            "]".red().bold(),
            self.lexeme,
        )
    }
}

#[cfg(test)]
mod smoke_token {
    use super::*;

    #[test]
    fn new_token() {
        let token = Token::new(TokenKind::WHITESPACE, " ".into(), Span::new(0..1));
        assert_eq!(token.kind(), &TokenKind::WHITESPACE);
        assert_eq!(token.lexeme(), " ");
        assert_eq!(token.span(), &Span::new(0..1));
    }

    #[test]
    fn builder_token() {
        let token = Token::builder()
            .kind(TokenKind::WHITESPACE)
            .lexeme(" ".into())
            .span(Span::new(0..1))
            .build();

        assert_eq!(token.kind(), &TokenKind::WHITESPACE);
        assert_eq!(token.lexeme(), " ");
        assert_eq!(token.span(), &Span::new(0..1));
    }
}
