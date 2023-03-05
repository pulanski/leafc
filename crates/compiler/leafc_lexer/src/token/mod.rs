#![doc = TOKEN_README!()]
#![allow(dead_code)] // TODO: Remove this.

use std::fmt;

use leafc_cfg::lang::LanguageKind;
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

// #[cfg(feature = "multi-threaded")]
// use std::sync::Arc;

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

impl Token {
    /// Creates a new **EOF** token (i.e. the **end-of-file** token).
    /// This token is **always** the **last** token in the input string.
    #[inline]
    #[allow(non_snake_case)]
    pub fn EOF() -> Self {
        Self::new(TokenKind::EOF, SmolStr::new("EOF_TOMBSTONE"), Span::new(0..0))
    }

    /// Returns `true` if the token is considered **valid** in the context of
    /// the **English** language. This is used to **filter** out tokens that
    /// are **not** valid in the **English** language.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use leafc_lexer::Token;
    ///
    /// let token = Token::new(TokenKind::FN_KW, "fn".into(), Span::new(0..2));
    /// assert!(token.is_english());
    ///
    /// let token2 = Token::new(TokenKind::FN_KW, "función".into(), Span::new(0..7));
    /// assert!(!token2.is_english());
    /// ```
    pub fn is_english(&self) -> bool {
        match self.kind() {
            // Generic tokens
            generic_tokens!() => true,
            _ => false, /* temp
                         * // ----------------------------------------------------------------
                         * // Generic tokens
                         * // ----------------------------------------------------------------
                         * TokenKind::ERROR => true,
                         * TokenKind::WHITESPACE => true,
                         * TokenKind::IDENTIFIER => true,
                         * TokenKind::COMMENT => true,
                         * TokenKind::DOC_COMMENT => true,
                         * TokenKind::RUNE => true,
                         * TokenKind::STRING => true,
                         * TokenKind::RAW_STRING => true,
                         * TokenKind::INTEGER => true,
                         * TokenKind::FLOAT => true,
                         * TokenKind::LIFETIME => true,
                         * TokenKind::INTEGER_SUP => true,
                         * TokenKind::FLOAT_SUP => true,
                         * TokenKind::PI => true,
                         * TokenKind::EULER => true,
                         * TokenKind::PHI => true,
                         * TokenKind::TAU => true,
                         * TokenKind::CATALAN => true,
                         * TokenKind::EULERGAMMA => true,
                         * TokenKind::INF => true,
                         * TokenKind::NAN => true,
                         * TokenKind::DEFINE => true,
                         * TokenKind::PLUS => true,
                         * TokenKind::MINUS => true,
                         * TokenKind::STAR => true,
                         * TokenKind::SLASH => true,
                         * TokenKind::PERCENT => true,
                         * TokenKind::CARET => true,
                         * TokenKind::BANG => true,
                         * TokenKind::AMPERSAND => true,
                         * TokenKind::PIPE => true,
                         * TokenKind::DOUBLE_AMPERSAND => true,
                         * TokenKind::DOUBLE_PIPE => true,
                         * TokenKind::SHL => true,
                         * TokenKind::SHR => true,
                         * TokenKind::PLUS_EQ => true,
                         * TokenKind::MINUS_EQ => true,
                         * TokenKind::STAR_EQ => true,
                         * TokenKind::SLASH_EQ => true,
                         * TokenKind::PERCENT_EQ => true,
                         * TokenKind::CARET_EQ => true,
                         * TokenKind::AMPERSAND_EQ => true,
                         * TokenKind::PIPE_EQ => true,
                         * TokenKind::SHL_EQ => true,
                         * TokenKind::SHR_EQ => true,
                         * TokenKind::EQ => true,
                         * TokenKind::EQEQ => true,
                         * TokenKind::NE => true,
                         * TokenKind::GT => true,
                         * TokenKind::LT => true,
                         * TokenKind::GE => true,
                         * TokenKind::LE => true,
                         * TokenKind::AT => true,
                         * TokenKind::UNDERSCORE => true,
                         * TokenKind::DOT => true,
                         * TokenKind::DOTDOT => true,
                         * TokenKind::DOTDOTEQ => true,
                         * TokenKind::COMMA => true,
                         * TokenKind::SEMICOLON => true,
                         * TokenKind::COLON => true,
                         * TokenKind::PATHSEP => true,
                         * TokenKind::RARROW => true,
                         * TokenKind::FATARROW => true,
                         * TokenKind::HASH => true,
                         * TokenKind::DOLLAR => true,
                         * TokenKind::QMARK => true,
                         * TokenKind::TILDE => true,
                         * TokenKind::L_BRACKET => true,
                         * TokenKind::R_BRACKET => true,
                         * TokenKind::L_PAREN => true,
                         * TokenKind::R_PAREN => true,
                         * TokenKind::L_BRACE => true,
                         * TokenKind::R_BRACE => true,
                         * TokenKind::L_PAREN_SUPERSCRIPT => true,
                         * TokenKind::R_PAREN_SUPERSCRIPT => true,
                         * TokenKind::L_ARROW => true,
                         * TokenKind::DOUBLE_STAR => true, */

                        /* // ----------------------------------------------------------------
                         * // Language-specific tokens
                         * // ----------------------------------------------------------------
                         * TokenKind::ABSTRACT_KW => match self.lexeme().as_str() {
                         *     "abstract" => true,
                         *     _ => false,
                         * },
                         * TokenKind::ASYNC_KW => match self.lexeme().as_str() {
                         *     "async" => true,
                         *     _ => false,
                         * },
                         * TokenKind::AWAIT_KW => match self.lexeme().as_str() {
                         *     "await" => true,
                         *     _ => false,
                         * },
                         * TokenKind::CASE_KW => match self.lexeme().as_str() {
                         *     "case" => true,
                         *     _ => false,
                         * },
                         * TokenKind::EXTERN_KW => match self.lexeme().as_str() {
                         *     "extern" => true,
                         *     _ => false,
                         * },
                         * TokenKind::FINAL_KW => match self.lexeme().as_str() {
                         *     "final" => true,
                         *     _ => false,
                         * },
                         * TokenKind::IMPORT_KW => match self.lexeme().as_str() {
                         *     "import" => true,
                         *     _ => false,
                         * },
                         * TokenKind::LET_KW => match self.lexeme().as_str() {
                         *     "let" => true,
                         *     _ => false,
                         * },
                         * TokenKind::AND_KW => match self.lexeme().as_str() {
                         *     "and" => true,
                         *     _ => false,
                         * },
                         * TokenKind::AS_KW => match self.lexeme().as_str() {
                         *     "as" => true,
                         *     _ => false,
                         * },
                         * TokenKind::BREAK_KW => match self.lexeme().as_str() {
                         *     "break" => true,
                         *     _ => false,
                         * },
                         * TokenKind::CONST_KW => match self.lexeme().as_str() {
                         *     "const" => true,
                         *     _ => false,
                         * },
                         * TokenKind::CONTINUE_KW => match self.lexeme().as_str() {
                         *     "continue" => true,
                         *     _ => false,
                         * },
                         * TokenKind::DEFAULT_KW => match self.lexeme().as_str() {
                         *     "default" => true,
                         *     _ => false,
                         * },
                         * TokenKind::DEFER_KW => match self.lexeme().as_str() {
                         *     "defer" => true,
                         *     _ => false,
                         * },
                         * TokenKind::DO_KW => match self.lexeme().as_str() {
                         *     "do" => true,
                         *     _ => false,
                         * },
                         * TokenKind::DYN_KW => match self.lexeme().as_str() {
                         *     "dyn" => true,
                         *     _ => false,
                         * },
                         * TokenKind::ELSE_KW => match self.lexeme().as_str() {
                         *     "else" => true,
                         *     _ => false,
                         * },
                         * TokenKind::ENUM_KW => match self.lexeme().as_str() {
                         *     "enum" => true,
                         *     _ => false,
                         * },
                         * TokenKind::FALLTHROUGH_KW => match self.lexeme().as_str() {
                         *     "fallthrough" => true,
                         *     _ => false,
                         * },
                         * TokenKind::FALSE_KW => match self.lexeme().as_str() {
                         *     "false" => true,
                         *     _ => false,
                         * },
                         * TokenKind::FN_KW => match self.lexeme().as_str() {
                         *     "fn" => true,
                         *     _ => false,
                         * },
                         * TokenKind::FOR_KW => match self.lexeme().as_str() {
                         *     "for" => true,
                         *     _ => false,
                         * },
                         * TokenKind::IF_KW => match self.lexeme().as_str() {
                         *     "if" => true,
                         *     _ => false,
                         * },
                         * TokenKind::IMPL_KW => match self.lexeme().as_str() {
                         *     "impl" => true,
                         *     _ => false,
                         * },
                         * TokenKind::IN_KW => match self.lexeme().as_str() {
                         *     "in" => true,
                         *     _ => false,
                         * },
                         * TokenKind::IS_KW => match self.lexeme().as_str() {
                         *     "is" => true,
                         *     _ => false,
                         * },
                         * TokenKind::ISNT_KW => match self.lexeme().as_str() {
                         *     "isnt" => true,
                         *     _ => false,
                         * },
                         * TokenKind::LOOP_KW => match self.lexeme().as_str() {
                         *     "loop" => true,
                         *     _ => false,
                         * },
                         * TokenKind::MATCH_KW => match self.lexeme().as_str() {
                         *     "match" => true,
                         *     _ => false,
                         * },
                         * TokenKind::MISSING_KW => match self.lexeme().as_str() {
                         *     "missing" => true,
                         *     _ => false,
                         * },
                         * TokenKind::MOD_KW => match self.lexeme().as_str() {
                         *     "mod" => true,
                         *     _ => false,
                         * },
                         * TokenKind::MOVE_KW => match self.lexeme().as_str() {
                         *     "move" => true,
                         *     _ => false,
                         * },
                         * TokenKind::MUT_KW => match self.lexeme().as_str() {
                         *     "mut" => true,
                         *     _ => false,
                         * },
                         * TokenKind::NOT_KW => match self.lexeme().as_str() {
                         *     "not" => true,
                         *     _ => false,
                         * },
                         * TokenKind::OR_KW => match self.lexeme().as_str() {
                         *     "or" => true,
                         *     _ => false,
                         * },
                         * TokenKind::PACKAGE_KW => match self.lexeme().as_str() {
                         *     "package" => true,
                         *     _ => false,
                         * },
                         * TokenKind::PUB_KW => match self.lexeme().as_str() {
                         *     "pub" => true,
                         *     _ => false,
                         * },
                         * TokenKind::RETURN_KW => match self.lexeme().as_str() {
                         *     "return" => true,
                         *     _ => false,
                         * },
                         * TokenKind::SELF_VALUE_KW => match self.lexeme().as_str() {
                         *     "self" => true,
                         *     _ => false,
                         * },
                         * TokenKind::SELF_TYPE_KW => match self.lexeme().as_str() {
                         *     "Self" => true,
                         *     _ => false,
                         * },
                         * TokenKind::STATIC_KW => match self.lexeme().as_str() {
                         *     "static" => true,
                         *     _ => false,
                         * },
                         * TokenKind::STRUCT_KW => match self.lexeme().as_str() {
                         *     "struct" => true,
                         *     _ => false,
                         * },
                         * TokenKind::SUPER_KW => match self.lexeme().as_str() {
                         *     "super" => true,
                         *     _ => false,
                         * },
                         * TokenKind::TRAIT_KW => match self.lexeme().as_str() {
                         *     "trait" => true,
                         *     _ => false,
                         * },
                         * TokenKind::TRUE_KW => match self.lexeme().as_str() {
                         *     "true" => true,
                         *     _ => false,
                         * },
                         * TokenKind::TYPE_KW => match self.lexeme().as_str() {
                         *     "type" => true,
                         *     _ => false,
                         * },
                         * TokenKind::UNSAFE_KW => match self.lexeme().as_str() {
                         *     "unsafe" => true,
                         *     _ => false,
                         * },
                         * TokenKind::USE_KW => match self.lexeme().as_str() {
                         *     "use" => true,
                         *     _ => false,
                         * },
                         * TokenKind::WHERE_KW => match self.lexeme().as_str() {
                         *     "where" => true,
                         *     _ => false,
                         * },
                         * TokenKind::WHILE_KW => match self.lexeme().as_str() {
                         *     "while" => true,
                         *     _ => false,
                         * },
                         * TokenKind::YIELD_KW => match self.lexeme().as_str() {
                         *     "yield" => true,
                         *     _ => false,
                         * }, */
        }
    }

    fn pretty_print(&self) -> String {
        format!(
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lexeme())
    }
    // fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //     write!(
    //         f,
    //         "{} {} {}{}{} {}",
    //         self.kind.cyan(),
    //         "@".black().italic(),
    //         "[".red().bold(),
    //         self.span,
    //         "]".red().bold(),
    //         self.lexeme,
    //     )
    // }
}

#[cfg(test)]
mod smoke_token {
    use super::*;
    use pretty_assertions_sorted::assert_eq;
    use rstest::rstest;

    // TODO: create a test fixture for generic token and then use it in the test
    // cases for each language-specific check (e.g. `is_english` or `is_german`)

    #[rstest]
    #[case(TokenKind::WHITESPACE, " ", Span::new(0..1))]
    #[case(TokenKind::IDENTIFIER, "foo", Span::new(0..3))]
    #[case(TokenKind::ABSTRACT_KW, "abstract", Span::new(0..8))]
    #[case(TokenKind::AS_KW, "as", Span::new(0..2))]
    #[case(TokenKind::BREAK_KW, "break", Span::new(0..5))]
    #[case(TokenKind::CONST_KW, "const", Span::new(0..5))]
    #[case(TokenKind::CONTINUE_KW, "continue", Span::new(0..8))]
    // #[case(TokenKind::CRATE_KW, "crate", Span::new(0..5))]
    #[case(TokenKind::ELSE_KW, "else", Span::new(0..4))]
    #[case(TokenKind::ENUM_KW, "enum", Span::new(0..4))]
    #[case(TokenKind::EXTERN_KW, "extern", Span::new(0..6))]
    #[case(TokenKind::FALSE_KW, "false", Span::new(0..5))]
    #[case(TokenKind::FN_KW, "fn", Span::new(0..2))]
    #[case(TokenKind::FOR_KW, "for", Span::new(0..3))]
    #[case(TokenKind::IF_KW, "if", Span::new(0..2))]
    #[case(TokenKind::IMPL_KW, "impl", Span::new(0..4))]
    #[case(TokenKind::IN_KW, "in", Span::new(0..2))]
    #[case(TokenKind::LET_KW, "let", Span::new(0..3))]
    #[case(TokenKind::LOOP_KW, "loop", Span::new(0..4))]
    #[case(TokenKind::MATCH_KW, "match", Span::new(0..5))]
    #[case(TokenKind::MOD_KW, "mod", Span::new(0..3))]
    #[case(TokenKind::MOVE_KW, "move", Span::new(0..4))]
    #[case(TokenKind::MUT_KW, "mut", Span::new(0..3))]
    // #[case(TokenKind::REF_KW, "ref", Span::new(0..3))]
    #[case(TokenKind::RETURN_KW, "return", Span::new(0..6))]
    #[case(TokenKind::SELF_TYPE_KW, "self", Span::new(0..4))]
    #[case(TokenKind::SELF_VALUE_KW, "Self", Span::new(0..4))]
    #[case(TokenKind::STATIC_KW, "static", Span::new(0..6))]
    #[case(TokenKind::STRUCT_KW, "struct", Span::new(0..6))]
    #[case(TokenKind::SUPER_KW, "super", Span::new(0..5))]
    #[case(TokenKind::TRAIT_KW, "trait", Span::new(0..5))]
    #[case(TokenKind::TRUE_KW, "true", Span::new(0..4))]
    #[case(TokenKind::TYPE_KW, "type", Span::new(0..4))]
    #[case(TokenKind::UNSAFE_KW, "unsafe", Span::new(0..6))]
    #[case(TokenKind::USE_KW, "use", Span::new(0..3))]
    #[case(TokenKind::WHERE_KW, "where", Span::new(0..5))]
    #[case(TokenKind::WHILE_KW, "while", Span::new(0..5))]
    #[case(TokenKind::YIELD_KW, "yield", Span::new(0..5))]
    fn is_english_keyword(#[case] kind: TokenKind, #[case] lexeme: &str, #[case] span: Span) {
        let token = Token::new(kind, lexeme.into(), span);
        assert_eq!(token.is_english(), true);
    }

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

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "allocative", derive(Allocative))]
pub struct TokenSet {
    pub tokens: HashSet<Token>,
}

/// ## [**`LanguageToken`**][LanguageToken]
///
/// A **language token** is a **token** that **belongs** to a **spoken
/// language**. A **language token** is **used** to **check** the **validity**
/// of a **token** as it belongs to a **specific** spoken language.
///
/// # Example:
///
/// ```rust
/// use leafc_lexer::token::{
///     LanguageKind,
///     LanguageToken,
///     Token,
/// };
///
/// let token = LanguageToken::new(TokenKind::FN, LanguageKind::English);
///
/// assert_eq!(token.token(), TokenKind::IDENTIFIER);
/// assert_eq!(token.lang(), LanguageKind::English);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LanguageToken {
    /// The **token** that the language token represents (e.g. `continue` and
    /// `continuar` are both mapped to the `CONTINUE_KW` token).
    pub token: Token,
    /// The **language** that the token belongs to, if any. A token may not
    /// belong to any language (e.g. `+` is a valid token in any language).
    pub lang:  LanguageKind,
}

// impl From<Token> for LanguageToken {
//     fn from(token: Token) -> Self {
//         Self {
//             token,
//             lang: match token.lexeme().as_str() {
//                 "continue" => LanguageKind::English,
//                 "continuar" => LanguageKind::Spanish,
//                 _ => LanguageKind::Unknown,
//             },
//         }
//     }
// }

// ## [**`Checker`**][Checker]
//
// A **checker** is a **trait** is used within some **context** of the
// compiler to check the **validity** of source code. Examples of subsystems
// that use checkers are the **lexer** and the **parser**. Combined together,
// there is a more general **check pipeline** that is used to **check** the
// **validity** of the **source code** (i.e. a **proof of correctness** that
// the given **source code** will **compile**).
//
// The **check pipeline** is **composed** of a series of **passes** that
// **check** the **source code** for **errors** at various levels of **semantic
// meaning** and associated internal **representation**. The **check pipeline**
// is **used** to **check** the **source code** before it is **compiled** (i.e.
// before **codegen** occurs).
//
// This is effectively the **heart** of the **analysis** phase of the
// **compiler**. Furthermore, this is the ideal place to **emit** any
// **diagnostics** that are **generated** during the **analysis** phase, and
// , in general, is the preferred way to **develop** the **compiler**, as it
// allows for incremental watchmode compilation times (_rust build times are
// notoriously slow_).
//
// // TODO: in the core database crate, module name should be `check` within
// an `analysis` module
// trait Checker {
//     // fn check(&self, source_file: &SourceFile) -> Result<LeafcDiagnostics>;

//     fn check(&self, token: &Token) -> bool;
//     // fn check(&self, token: &Token) -> (bool, LeafcDiagnostics); // TODO
// }
