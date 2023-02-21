// #[cfg_attr(aquamarine, doc)]
// #![doc = include_str!("../LEXER.md")]

use getset::Getters;
use leafc_utils::codemap::Span;
use logos::Logos;
use smol_str::SmolStr;
use std::fmt;

use crate::token::{
    Token,
    TokenKind,
};

/// Performs a **lossy** lexing of the input string (i.e. a **minimal
/// representation** of the input text source).
///
/// # Example:
///
/// ```rust,ignore
/// use leafc_lexer::lossy_lex;
///
/// let tokens = lossy_lex("fn main() { println!(\"Hello, world!\"); }");
///
/// assert_eq!(tokens.to_string(), r#"FN_KW @ [0..2] fn
/// WHITESPACE @ [2..3]
/// IDENTIFIER @ [3..7] main
/// L_PAREN @ [7..8] (
/// R_PAREN @ [8..9] )
/// WHITESPACE @ [9..10]
/// L_BRACE @ [10..11] {
/// WHITESPACE @ [11..12]
/// IDENTIFIER @ [12..19] println
/// L_PAREN @ [19..20] (
/// STRING @ [20..34] "Hello world!"
/// R_PAREN @ [34..35] )
/// WHITESPACE @ [35..36]
/// R_BRACE @ [36..37] }
/// "#);
pub fn lossy_lex(input: &str) -> TokenStream {
    TokenStream::new(input, false)
}

/// Performs a **lossless** lexing of the input string (i.e. a **full fidelity**
/// representation of the input text source).
///
/// # Example:
///
/// ```rust,ignore
/// use leafc_lexer::lossless_lex;
///
/// let tokens = lossless_lex("fn main() { println!(\"Hello, world!\"); }");
///
/// assert_eq!(tokens.to_string(), r#"FN_KW @ [0..2] fn
/// WHITESPACE @ [2..3]
/// IDENTIFIER @ [3..7] main
/// L_PAREN @ [7..8] (
/// R_PAREN @ [8..9] )
/// WHITESPACE @ [9..10]
/// L_BRACE @ [10..11] {
/// WHITESPACE @ [11..12]
/// IDENTIFIER @ [12..19] println
/// L_PAREN @ [19..20] (
/// STRING @ [20..34] "Hello world!"
/// R_PAREN @ [34..35] )
/// WHITESPACE @ [35..36]
/// R_BRACE @ [36..37] }
/// "#);
pub fn lossless_lex(input: &str) -> TokenStream {
    TokenStream::new(input, true)
}

#[derive(Debug, PartialEq, Eq, Hash, Getters)]
#[getset(get = "pub")]
pub struct TokenStream {
    /// The **tokens** of the input string.
    tokens: Vec<Token>,

    /// The **current span** of the lexer in the input string. (MAYBE)
    // curr_span: Span,

    /// The **current line** of the lexer in the input string.
    ///
    /// This is used to provide better error messages (e.g. `unexpected token
    /// on line 3` along with the line of code that caused the error and the
    /// span in the source code)
    ///
    /// Stores the line where the first error token occurred, if any (calculated
    /// from the source span during lexing).
    curr_line: Option<usize>,

    /// The **current offset** of the lexer in the input string.
    ///
    /// This is used to provide better error messages (e.g. `unexpected token
    /// on line 3 column 17` along with the line of code that caused the error
    /// and the span in the source code)
    ///
    /// Stores the offset where the first error token occurred, if any
    /// (calculated from the source span during lexing).
    curr_offset: Option<usize>,

    /// Whether or not the token stream is **lossless** (e.g. a **full
    /// fidelity** representation of the input text source).
    lossless: bool,
}

impl fmt::Display for TokenStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for token in &self.tokens[..self.tokens.len() - 1] {
            writeln!(f, "{token}")?;
        }
        write!(f, "{}", self.tokens.last().unwrap());

        Ok(())
    }
}

impl TokenStream {
    pub fn new(input: &str, lossless: bool) -> Self {
        let mut lex = TokenKind::lexer(input);

        let mut tokens = Vec::new();

        while let Some(token) = lex.next() {
            if !lossless && (token.is_whitespace() || token.is_comment()) {
                continue;
            }

            tokens.push(Token::new(
                token,
                SmolStr::new(lex.slice()),
                Span::new(lex.span().start, lex.span().end),
            ));
        }

        Self { tokens, curr_line: None, curr_offset: None, lossless }
    }

    // TODO: implement an iterator for token stream
}
