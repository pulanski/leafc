#![allow(dead_code, unused)]
use std::fmt;

// TOOD: remove this
use crate::token::TokenKind;
use leafc_utils::location::Span;
use logos::Logos;
use smol_str::SmolStr;

// fn lex(input: &str) -> Vec<TokenKind> {
//     let mut lex = TokenKind::lexer(input);

//     let mut tokens = Vec::new();

//     // while let Some(token) = lex.next() {
//     //     tokens.push(token);
//     // }
// }

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TokenStream {
    /// The **lexer** that will be used to lex the input string.
    // lexer: Lexer<'a, Token>,
    tokens: Vec<Token>,

    /// The **current line** of the lexer in the input string.
    /// This is used to provide better error messages.
    line: usize,
    /// The **current offset** of the lexer in the input string.
    offset: usize,

    // current: Option<TokenKind>,
    /// Whether or not the token stream is **lossless** (e.g. a **full fidelity**
    /// representation of the input text source).
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
/// // TODO: add example
/// // `use` is a keyword,
/// // `std` is an identifier,
/// // and `;` is punctuation.
/// use std::fmt;
/// ```
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Token {
    /// The **kind** of the token (i.e. the **type** of the token)
    kind: TokenKind,

    /// The **lexeme** of the token (i.e. the **text** that the token represents)
    lexeme: SmolStr,

    /// The **span** of the token (i.e. the **location** of the token in the input string)
    span: Span,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} @ ({}) {}", self.kind, self.span, self.lexeme)
    }
}

impl TokenStream {
    pub fn new(input: &str, lossless: bool) -> Self {
        let mut lex = TokenKind::lexer(input);

        let mut tokens = Vec::new();

        while let Some(token) = lex.next() {
            if !lossless && token.is_whitespace() {
                continue;
            }

            tokens.push(Token {
                kind: token,
                lexeme: SmolStr::new(lex.slice()),
                span: Span::new(lex.span().start, lex.span().end),
            });
        }

        Self { tokens, line: 1, offset: 0, lossless }
    }

    // fn peek(&self) -> Option<Token> {
    //     self.lexer.clone().peekable().peek().as_ref().copied().copied()
    // }

    // fn peek_nth(&self, n: usize) -> Option<Token> {
    //     debug_assert!(n < 3);
    //     self.lexer.clone().peekable().nth(n).as_ref().copied()
    // }

    // fn next(&mut self) -> Option<Token> {
    //     self.lexer.next()
    // }

    // fn has_next(&self) -> bool {
    //     self.lexer.clone().peekable().peek().is_some()
    // }

    // fn expect(&mut self, token: Token) -> Result<(), ParseError> {
    //     if self.current == Some(&token) {
    //         self.advance();
    //         Ok(())
    //     } else {
    //         // Err(ParseError::UnexpectedToken { expected: token, found: self.current })
    //     }
    // }

    // fn expect_one_of(&mut self, tokens: &[Token]) -> Result<(), ParseError> {
    //     if tokens.contains(self.current()) {
    //         self.advance();
    //         Ok(())
    //     } else {
    //         // Err(ParseError::UnexpectedToken { expected: Token::ERROR, found: self.current })
    //     }
    // }
}
