// #[cfg_attr(aquamarine, doc)]
// #![doc = include_str!("../LEXER.md")]

use getset::{
    Getters,
    MutGetters,
    Setters,
};
use leafc_utils::codemap::Span;
use logos::Logos;
use smartstring::alias::String;
use smol_str::SmolStr;
use std::{
    collections::VecDeque,
    fmt,
    ops::{
        Add,
        AddAssign,
    },
};

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

/// A **token offset** is a **pointer** to a **token** in a **token stream**.
/// It is used to **index** into a **token stream**.
///
/// # Example:
///
/// ```rust
/// use leafc_lexer::TokenOffset;
///
/// let offset = TokenOffset(0);
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct TokenOffset(usize);

impl From<usize> for TokenOffset {
    fn from(offset: usize) -> Self {
        Self(offset)
    }
}

impl From<TokenOffset> for usize {
    fn from(offset: TokenOffset) -> Self {
        offset.0
    }
}

impl Add<usize> for TokenOffset {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl AddAssign<usize> for TokenOffset {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl TokenOffset {
    /// Creates a new **token offset**.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use leafc_lexer::TokenOffset;
    ///
    /// let offset = TokenOffset::new(0);
    /// ```
    pub fn new(offset: usize) -> Self {
        Self(offset)
    }

    /// Returns the **token offset** as a **`usize`**.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use leafc_lexer::TokenOffset;
    ///
    /// let offset = TokenOffset::new(0);
    ///
    /// assert_eq!(offset.as_usize(), 0);
    /// ```
    pub fn as_usize(self) -> usize {
        self.0
    }

    /// Increments the **token offset** by `1`.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use leafc_lexer::TokenOffset;
    ///
    /// let mut offset = TokenOffset::new(0);
    ///
    /// // Increment the token offset by 1.
    /// offset.inc();
    ///
    /// assert_eq!(offset.as_usize(), 1);
    /// ```
    pub fn inc(&mut self) {
        self.0 += 1;
    }
}

/// A **token stream** is a **stream of tokens** that the lexer produces.
/// It is a **lossy** or **lossless** representation of the input text source.
/// It is **lossy** if it does not contain whitespace or comments (e.g. a
/// **minimal representation** of the input text source). Otherwise, it is
/// **lossless** (e.g. a **full fidelity** representation of the input text
/// source).
#[derive(Debug, PartialEq, Eq, Hash, Clone, Getters, MutGetters, Setters)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct TokenStream {
    /// The **input** string that the lexer is lexing.
    input: String,

    /// The **tokens** of the input string.
    tokens: VecDeque<Token>,

    /// The **current token** the lexer is looking at.
    cursor: TokenOffset,

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
        for token in &self.tokens {
            writeln!(f, "{token}")?;
        }

        Ok(())
        // for token in &self.tokens[..self.tokens.len() - 1] {
        //     writeln!(f, "{token}")?;
        // }
        // write!(f, "{}", self.tokens[self.tokens.len() - 1])
    }
}

impl TokenStream {
    pub fn new(input: &str, lossless: bool) -> Self {
        let mut lex = TokenKind::lexer(input);

        let mut tokens = VecDeque::new();

        while let Some(token) = lex.next() {
            if !lossless && (token.is_whitespace() || token.is_comment()) {
                continue;
            }

            tokens.push_back(Token::new(token, SmolStr::new(lex.slice()), Span::new(lex.span())));
        }

        Self {
            input: input.to_string().into(),
            tokens,
            cursor: TokenOffset::new(0),
            curr_line: None,
            curr_offset: None,
            lossless,
        }
    }

    /// Peek at the **next token** in the token stream.
    ///
    /// # Example:
    ///
    /// ```rust,ignore
    /// use leafc_lexer::lossy_lex;
    ///
    /// let tokens = lossy_lex("fn main() { println!(\"Hello, world!\"); }");
    ///
    /// assert_eq!(tokens.peek(), Some(&Token::new(TokenKind::FnKw, "fn".into(), Span::new(0..2))));
    /// ```
    #[inline]
    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.cursor().as_usize())
    }

    /// Returns the token at the **given index** in the token stream.
    /// Returns `None` if the index is out of bounds.
    ///
    /// # Example:
    /// ```rust,ignore
    /// use leafc_lexer::lossy_lex;
    /// use leafc_token::TokenKind;
    /// use leafc_utils::codemap::Span;
    ///
    /// let tokens = lossy_lex("fn main() { println!(\"Hello, world!\"); }");
    ///
    /// assert_eq!(tokens.at(0), Some(&Token::new(TokenKind::FnKw, "fn".into(), Span::new(0..2))));
    /// assert_eq!(tokens.at(1), Some(&Token::new(TokenKind::Identifier, "main".into(), Span::new(3..7))));
    /// ```
    #[inline]
    pub fn at(&self, index: usize) -> Option<&Token> {
        self.tokens.get(index)
    }

    /// Returns the **number of tokens** in the token stream.
    /// This is equivalent to `tokens.len()`.
    #[inline]
    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    /// Returns `true` if the token stream is **empty**.
    /// This is equivalent to `tokens.is_empty()`.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    /// **Increment** the cursor of the token stream.
    #[inline]
    pub fn inc_cursor(&mut self) {
        self.cursor.inc();
    }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let cursor = (*self.cursor()).as_usize();

        if cursor >= self.tokens.len() {
            return None;
        }

        // TODO: refactor to not use unwrap and clone here
        let token = self.tokens.get(cursor).unwrap().clone();
        self.inc_cursor();
        Some(token)

        // let token = self.tokens.get(cursor)?;

        // self.inc_cursor();
        // Some(*token)
    }
}
