#![doc = TOKEN_README!()]

use std::fmt;

use derive_new::new;
use leafc_utils::Span;
use owo_colors::OwoColorize;
use smol_str::SmolStr;

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
#[derive(Debug, PartialEq, Eq, Hash, new)]
pub struct Token {
    /// The **kind** of the token (i.e. `WHITESPACE`, `IDENTIFIER`, etc.)
    kind:   TokenKind,
    /// The **lexeme** of the token (i.e. the **text** that the token
    /// represents)
    lexeme: SmolStr,
    /// The **span** of the token (i.e. the **location** of the token in the
    /// input string)
    span:   Span,
}

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
