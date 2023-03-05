use std::{
    cell::RefCell,
    collections::VecDeque,
    rc::Rc,
};

use getset::{
    Getters,
    MutGetters,
    Setters,
};
use leafc_diagnostics::errors::SyntaxError;
use leafc_lexer::{
    lexer::{
        TokenOffset,
        TokenStream,
    },
    token::Token,
    TokenKind,
};
use leafc_syntax::SyntaxTreeBuilder;

// use crate::event::Event;

/// The **parser** for the **Leaf programming language**. This is the main
/// entry point for parsing a source file. The parser is responsible for
/// converting a stream of tokens into an abstract syntax tree (_AST_). The
/// parser is also responsible for reporting any syntax errors encountered
/// during parsing. The parser is implemented as a **recursive descent parser**.
/// This means that the parser is implemented as a set of mutually recursive
/// functions, where each function is responsible for parsing a particular
/// syntactic construct.
///
/// # Example:
///
/// ```rust,ignore
/// use leafc_parser::Parser;
///
/// let parser = Parser::new("fn main() { println!(\"Hello, world!\"); }", false);
/// let ast = parser.parse();
/// ```
#[derive(Debug, Clone, Getters, MutGetters, Setters)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "allocative", derive(Allocative))]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct Parser {
    /// The **stream of tokens** to parse.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use leafc_parser::Parser;
    ///
    /// let parser = Parser::new("fn main() { println!(\"Hello, world!\"); }", false);
    ///
    /// // Retrieve the token stream from the parser.
    /// let tokens = parser.tokens();
    ///
    /// // Retrieve the token stream from the parser mutably.
    /// let mut mut_tokens = parser.tokens_mut();
    ///
    /// // Set the token stream of the parser.
    /// parser.set_tokens(tokens);
    /// ```
    tokens: TokenStream,

    /// The [`SyntaxTreeBuilder`] used to build the underlying syntax tree
    /// created by the parser. This has the benefit of structural sharing.
    ///
    /// **NOTE**: This is wrapped in an [`Rc`] and [`RefCell`]
    /// to allow for **multiple mutable borrows**.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use leafc_parser::Parser;
    ///
    /// let parser = Parser::new("fn main() { println!(\"Hello, world!\"); }", false);
    ///
    /// // Retrieve the syntax tree builder from the parser.
    /// let tree_builder = parser.tree_builder();
    ///
    /// // Retrieve the syntax tree builder from the parser mutably.
    /// let mut mut_tree_builder = parser.tree_builder_mut();
    ///
    /// // Set the syntax tree builder of the parser.
    /// parser.set_tree_builder(tree_builder);
    /// ```
    tree_builder: Rc<RefCell<SyntaxTreeBuilder>>,

    /// The list of **syntax errors** encountered during parsing.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use leafc_parser::Parser;
    ///
    /// let parser = Parser::new("fn main() { println!(\"Hello, world!\"); }", false);
    ///
    /// // Retrieve the list of syntax errors from the parser.
    /// let errors = parser.errors();
    ///
    /// // Retrieve the list of syntax errors from the parser mutably.
    /// let mut mut_errors = parser.errors_mut();
    ///
    /// // Set the list of syntax errors of the parser.
    /// parser.set_errors(errors);
    /// ```
    errors: VecDeque<SyntaxError>,
    // The list of events that have occurred during parsing.
    // events: VecDeque<Option<Event>>, // TODO: add implementation for this
}

impl Parser {
    /// Creates a new [`Parser`] from the given input string.
    /// The `lossless` parameter determines whether or not the parser should
    /// preserve whitespace and comments in the token stream.
    ///
    /// # Arguments
    ///
    /// * `input_str` - The input string to parse.
    /// * `lossless` - Whether or not the parser should preserve whitespace and
    ///  comments in the token stream.
    ///
    /// # Returns
    ///
    /// A new [`Parser`] instance.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use leafc_parser::Parser;
    ///
    /// // Create a new parser from the given input string. The parser will
    /// // preserve whitespace and comments in the token stream.
    /// let parser = Parser::new("fn main() { println!(\"Hello, world!\"); }", true);
    /// ```
    pub fn new(input_str: &str, lossless: bool) -> Self {
        let tokens = TokenStream::new(input_str, lossless);

        Self {
            tokens,
            tree_builder: Rc::new(RefCell::new(SyntaxTreeBuilder::new())),
            errors: VecDeque::new(),
        }
    }

    // -----------------------------------------------------------------------
    // Token retrieval methods
    // -----------------------------------------------------------------------

    /// Returns the current token in the [`TokenStream`] being parsed.
    /// If the cursor is at the end of the stream, then `None` is returned.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use leafc_parser::Parser;
    ///
    /// let parser = Parser::new("fn main() { println!(\"Hello, world!\"); }", false);
    ///
    /// // Retrieve the current token from the parser.
    /// let curr_token = parser.curr_token()?;
    /// assert_eq!(curr_token.kind(), TokenKind::FN_KW);
    /// ```
    #[inline]
    pub fn curr_token(&self) -> Option<&Token> {
        self.tokens.at(self.cursor().into())
    }

    /// The number of tokens in the [`TokenStream`] being parsed.
    /// This is useful for checking if the end of the stream has been reached.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use leafc_parser::Parser;
    ///
    /// let parser = Parser::new("fn main() { println!(\"Hello, world!\"); }", false);
    /// assert_eq!(parser.num_tokens(), 15);
    /// ```
    #[inline]
    pub fn num_tokens(&self) -> usize {
        self.tokens.len()
    }

    /// The current position of the cursor in the [`TokenStream`] being parsed.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use leafc_parser::Parser;
    ///
    /// let parser = Parser::new("fn main() { println!(\"Hello, world!\"); }", false);
    /// assert_eq!(parser.cursor(), TokenOffset::from(0));
    /// ```
    pub fn cursor(&self) -> TokenOffset {
        *self.tokens().cursor()
    }

    /// Returns the next token in the [`TokenStream`] being parsed. If the
    /// cursor is at the end of the stream, then `None` is returned.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use leafc_parser::Parser;
    ///
    /// let parser = Parser::new("1 + 2", false);
    ///
    /// // Retrieve the next token from the parser.
    /// assert_eq!(parser.next_token()?.kind(), TokenKind::INTEGER);
    ///
    /// // Retrieve the next token from the parser.
    /// assert_eq!(parser.next_token()?.kind(), TokenKind::PLUS);
    ///
    /// // Retrieve the next token from the parser.
    /// assert_eq!(parser.next_token()?.kind(), TokenKind::INTEGER);
    ///
    /// // There are no more tokens in the stream.
    /// assert_eq!(parser.next_token(), None);
    /// ```
    pub fn next_token(&mut self) -> Option<Token> {
        self.tokens_mut().next()
    }

    /// Returns the **nth** token ahead of the current token in the
    /// [`TokenStream`] being parsed. If the cursor is at the end of the
    /// stream, then `None` is returned.
    ///
    /// **NOTE**: This method only supports up to 3 tokens ahead.
    /// This is because the parser only needs to look ahead by 3 tokens
    /// at most.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of tokens ahead to look (_0 < n < 4_)
    pub fn nth_token_ahead(&self, n: usize) -> Option<&Token> {
        debug_assert!(n > 0 && n < 4);

        self.tokens.at((self.cursor() + n).into())
    }

    fn inc_cursor(&mut self) {
        self.tokens_mut().inc_cursor();
    }

    // -----------------------------------------------------------------------
    // Parser methods
    // -----------------------------------------------------------------------

    /// Consumes the next token in the [`TokenStream`] being parsed.
    pub fn pop(&mut self) -> Token {
        if let Some(token) = self.next_token() {
            token
        } else {
            Token::EOF()
        }
    }

    /// Consumes the token if it **matches** the **expected token kind**.
    ///
    /// # Arguments
    ///
    /// * `expected` - The expected token kind.
    ///
    /// # Returns
    ///
    /// `true` if the token matches the expected token kind.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let mut parser = Parser::new("fn main() { println!(\"Hello, world!\"); }", false);
    ///
    /// // The token kind is `TokenKind::Fn`
    /// let token = parser.pop();
    /// assert_eq!(token.kind(), &TokenKind::Fn);
    /// assert_eq!(parser.eat(&TokenKind::FN_KW), true);
    ///
    /// // The token kind is `TokenKind::IDENTIFIER`
    /// assert!(!parser.eat(&TokenKind::FN_KW));
    /// assert!(parser.eat(&TokenKind::IDENTIFIER));
    /// ```
    pub fn eat(&mut self, expected: &TokenKind) -> bool {
        if self.at_end() || !self.at(expected) {
            return false;
        }

        let token = self.pop();
        self.push_token(token);
        true
    }

    /// **Inserts** the given token into the underlying syntax tree
    /// being built by the parser.
    ///
    /// # Arguments
    ///
    /// * `token` - The token to insert into the underlying syntax tree.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let mut parser = Parser::new("fn main() { println!(\"Hello, world!\"); }", false);
    ///
    /// // The token kind is `TokenKind::FN_KW`
    /// let token = parser.pop();
    /// parser.push_token(token);
    /// ```
    pub fn push_token(&mut self, token: Token) {
        self.tree_builder().borrow_mut().add_token(&token);
    }

    /// Consumes the next token in the [`TokenStream`] being parsed.
    pub fn bump(&mut self, kind: &TokenKind) {
        // pub fn bump(&mut self, kind: &SyntaxKind) {
        debug_assert!(self.eat(kind));
    }

    /// Returns `true` if the current token the parser is looking at is a
    /// particular token kind.
    ///
    /// # Arguments
    ///
    /// * `kind` - The token kind to check for.
    ///
    /// # Returns
    ///
    /// `true` if the current token the parser is looking at is a particular
    /// token kind.
    ///
    /// # Examples
    ///
    /// ```
    /// use leafc_parser::parser::Parser;
    ///
    /// let mut parser = Parser::new("1 + 2", false);
    ///
    /// // The current token is a `TokenKind::INTEGER` token.
    /// assert!(parser.at(&TokenKind::INTEGER));
    /// assert!(!parser.at(&TokenKind::PLUS));
    /// ```
    pub fn at(&self, kind: &TokenKind) -> bool {
        self.curr_token().map_or(false, |token| token.kind() == kind)
    }

    /// Returns `true` if the current token the parser is looking at is any of
    /// the given token kinds.
    ///
    /// # Arguments
    ///
    /// * `kinds` - The token kinds to check for.
    ///
    /// # Returns
    ///
    /// `true` if the current token the parser is looking at is any of the
    /// given token kinds.
    ///
    /// # Examples
    ///
    /// ```
    /// use leafc_parser::parser::Parser;
    ///
    /// let mut parser = Parser::new("1 + 2", false);
    ///
    /// // The current token is a `TokenKind::INTEGER` token.
    /// assert!(parser.at_any(&[TokenKind::INTEGER, TokenKind::PLUS]));
    /// assert!(!parser.at_any(&[TokenKind::IDENTIFIER, TokenKind::MINUS]));
    /// ```
    pub fn at_any(&self, kinds: &[TokenKind]) -> bool {
        self.curr_token().map_or(false, |token| kinds.iter().any(|kind| token.kind() == kind))
    }

    // -----------------------------------------------------------------------
    // Utility methods
    // -----------------------------------------------------------------------

    /// Returns `true` if the cursor is at the end of the [`TokenStream`].
    /// This is useful for checking if the end of the stream has been reached.
    ///
    /// **NOTE**: This is the inverse of [`Parser::has_more_tokens`].
    ///
    /// # Returns
    ///
    /// `true` if the cursor is at the end of the [`TokenStream`].
    ///
    /// # Examples
    ///
    /// ```
    /// use leafc_parser::parser::Parser;
    ///
    /// let mut parser = Parser::new("1 + 2", false);
    ///
    /// // Parser is initialized with a cursor at the beginning of
    /// // the token stream (not at the end) so this will return false.
    /// assert!(!parser.at_end());
    /// ```
    pub fn at_end(&self) -> bool {
        let cursor: usize = self.cursor().into();
        cursor >= self.num_tokens()
    }

    /// Returns `true` if the cursor is **not** at the end of the
    /// [`TokenStream`]. This is useful for checking if the end of the stream
    /// has **not** been reached.
    ///
    /// **NOTE**: This is the inverse of [`Parser::at_end`].
    ///
    /// # Returns
    ///
    /// `true` if the cursor is **not** at the end of the [`TokenStream`].
    ///
    /// # Examples
    ///
    /// ```
    /// use leafc_parser::parser::Parser;
    ///
    /// let mut parser = Parser::new("1 + 2", false);
    ///
    /// // Parser is initialized with a cursor at the beginning of
    /// // the token stream.
    /// assert!(parser.has_more_tokens());
    /// ```
    pub fn has_more_tokens(&self) -> bool {
        !self.at_end()
    }

    /// The **number of syntax errors** encountered during parsing.
    /// This is useful for checking if the parser encountered any errors.
    ///
    /// # Returns
    ///
    /// The number of syntax errors encountered during parsing.
    ///
    /// # Examples
    ///
    /// ```
    /// use leafc_parser::parser::Parser;
    ///
    /// let mut parser = Parser::new("1 + 2", false);
    ///
    /// // Parser is initialized with 0 errors.
    /// assert_eq!(parser.num_errors(), 0);
    /// ```
    pub fn num_errors(&self) -> usize {
        self.errors().len()
    }

    /// Returns `true` if the parser encountered any syntax errors.
    ///
    /// # Returns
    ///
    /// `true` if the parser encountered any syntax errors.
    ///
    /// # Examples
    ///
    /// ```
    /// use leafc_parser::parser::Parser;
    ///
    /// let mut parser = Parser::new("1 + 2", false);
    ///
    /// // Parser has not encountered any errors yet.
    /// assert!(!parser.has_errors());
    /// ```
    pub fn has_errors(&self) -> bool {
        self.num_errors() > 0
    }
}

#[cfg(test)]
mod parser_test_suite {
    use super::*;

    #[test]
    fn test_parser_new() {
        let input_str = "1 + 2";
        let parser = Parser::new(input_str, false);

        assert_eq!(parser.num_tokens(), 3);
        assert_eq!(parser.cursor(), TokenOffset::from(0));
        assert_eq!(parser.num_errors(), 0);
        assert!(!parser.has_errors());

        // while parser.has
    }

    // #[test]
    // fn test_parser_curr_token() {
    //     let input_str = "1 + 2";
    //     let parser = Parser::new(input_str, false);

    //     assert_eq!(parser.curr_token().unwrap().kind, TokenKind::Number);
    // }
}
