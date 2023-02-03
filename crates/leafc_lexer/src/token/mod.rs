#![allow(dead_code)]
#![doc = include_str!("../../TOKEN.md")]

mod utils;

use logos::Logos;

/// ## [**`TokenGroup`**][TokenGroup]
///
/// These are a collection of **more coarse-grained sets** for specifying
/// particular subsets or groupings of tokens with **similar syntax and
/// semantics** from the perspective of the compiler. They include
///
/// - [**`Comments`**][TokenGroup::Comment]
/// - [**`Punctuation`**][TokenGroup::Punctuation]
/// - [**`Delimiters`**][TokenGroup::Delimiter]
/// - [**`Literals`**][TokenGroup::Literal]
/// - [**`Keywords`**][TokenGroup::Keyword]
/// - [**`Identifiers`**][TokenGroup::Identifier]
/// - [**`Errors`**][TokenGroup::Error]
///
/// This is useful for developing more complex **context-sensitive
/// analysis** and **error reporting** and changing between the
/// **concrete syntax tree** and the **abstract syntax tree**.
///
/// # Example:
///
/// ```rust
/// // `use` is a keyword,
/// // `std` is an identifier,
/// // and `;` is punctuation.
/// use std::fmt;
/// ```
pub enum TokenGroup {
    /// # **`Whitespace`**
    ///
    /// Whitespace is captured primarily for the purpose of constructing **high-fidelity,
    /// lossless syntax trees**. This idea is by no means new, as is in large part
    /// inspired by its implementations in systems such as [**rust-analyzer**]() (_rowan_), the [**Swift
    /// compiler**]() (_libsyntax_), and in [**C# compiler**]() (_Roslyn_).
    ///
    /// # Example:
    ///
    /// ```rust,ignore
    /// x := 1; // ` ` is whitespace.
    /// ```
    Whitespace,

    /// # **`Comment`**
    ///
    /// Comments are used to provide additional information to the
    /// reader of the source code. Comments are not executed by the
    /// compiler.
    ///
    /// Comments can be **single-line** or **multi-line**. Single-line
    /// comments can be prefixed by either `//` or '#' and multi-line comments are
    /// prefixed with `/*` and suffixed with `*/`.
    ///
    /// Additionally, comments can be **doc-comments**. Doc-comments are
    /// multi-line comments that are prefixed with `///` or `/**`.
    ///
    /// # Example:
    ///
    /// ```rust,ignore
    /// // This is a single-line comment.
    /// # This is also a single-line comment.
    ///
    /// /* This is a multi-line comment. */
    /// /*
    ///  * This is also a multi-line comment.
    ///  */
    ///
    /// /// This is a doc-comment.
    /// /** This is also a doc-comment. */
    /// ```
    Comment,

    /// # **`Punctuation`**
    ///
    /// Punctuation is used within a greater context to provide
    /// syntactic structure to the program. Examples of punctuation
    /// include `;`, `:`, `.` and `,`.
    ///
    /// # Example:
    ///
    /// ```rust
    /// use std::fmt; // `;` is punctuation.
    /// ```
    Punctuation,

    /// # **`Delimiter`**
    ///
    /// Delimiters are used to group tokens together. Examples of
    /// delimiters include `(`, `)`, `{`, `}`, `[` and `]`.
    ///
    /// # Example:
    ///
    /// ```rust,ignore
    /// x := (1 + 2) * 3; // `(` and `)` are delimiters.
    /// ```
    Delimiter,

    /// # **`Literal`**
    ///
    /// Literals are used to represent **fixed values** within the
    /// source code of a program.
    ///
    /// # Example:
    ///
    /// ```rust,ignore
    /// x := 1; // `1` is a literal.
    ///
    /// y := "Hello, world!"; // `"Hello, world!"` is also a literal.
    /// ```
    Literal,

    /// **`Keyword`**
    ///
    /// Words **built into the language** with special meaning in a
    /// particular context. This is a semantic definition and differs
    /// from names typically seen in a language's _standard library_.
    ///
    /// # Example:
    ///
    /// ```rust,ignore
    /// mut x := 1; // `mut` is a keyword.
    /// ```
    Keyword,

    /// **`Identifier`**
    ///
    /// A lexical token / symbol that names the language's entities.
    /// Identifiers are used to represent variables, functions, and other
    /// language entities.
    ///
    /// # Example:
    ///
    /// ```rust,ignore
    /// `x` //is an identifier.
    /// `main` // is an identifier.
    /// `num_threads` //is an identifier.
    /// ```
    Identifier,

    /// **ERROR**
    ///
    /// Any tokens which are not recognized by the lexer and as
    /// such are deemed invalid.
    ///
    /// # Examples
    ///
    /// ```text
    /// Token: Error,
    /// TokenType: ERROR,
    /// Lexeme: "foo"
    /// ```
    Error,
}

#[allow(non_camel_case_types)]
#[derive(Logos, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Token {
    #[error]
    // #[regex(r"[ \t\f]+", logos::skip)]
    ERROR,

    #[regex("//[^\r\n]*", priority = 2)]
    #[regex("//[^\n]*", priority = 1)]
    Comment,

    #[regex("///[^\r\n]*", priority = 4)]
    #[regex("///[^\n]*", priority = 3)]
    DocComment,

    #[regex(r"\s+")]
    WHITESPACE,

    #[regex("b?'[^']*'")]
    Rune,
    #[regex(r#"b?"(\\.|[^\\"])*""#)]
    String,

    #[token("inf")]
    #[token("NaN")]
    #[regex(r#"[+-]?[0-9][0-9_]*\.[0-9][0-9_]*([eE][+-]?[0-9][0-9_]*)?"#)]
    #[regex(
        r#"[+-]?0x[0-9a-fA-F][0-9a-fA-F_]*\.[0-9a-fA-F][0-9a-fA-F_]*([pP][+-]?[0-9][0-9_]?)?"#
    )]
    Float,

    #[regex("[+-]?[0-9][0-9_]*")]
    #[regex("[+-]?0b[0-1][0-1_]*")]
    #[regex("[+-]?0x[0-9a-fA-F][0-9a-fA-F_]*")]
    INTEGER,
    #[token("true")]
    #[token("false")]
    Bool,

    #[regex(r"[\p{XID_Start}\p{Emoji_Presentation}][\p{XID_Continue}\p{Emoji_Presentation}]*")]
    IDENT,

    #[token("fn")]
    FN_KW,
    #[token("import")]
    Import,
    #[token("let")]
    Let,
    #[token("type")]
    Type,
    #[token("enum")]
    Enum,
    #[token("trait")]
    Trait,
    #[token("const")]
    Const,
    #[token("extend")]
    Extend,
    #[token("with")]
    With,
    #[token("alias")]
    Alias,
    #[token("mut")]
    Mut,
    #[token("ref")]
    Ref,
    #[token("extern")]
    Extern,
    #[token("unsafe")]
    Unsafe,

    #[token("in")]
    In,
    #[token("is")]
    Is,
    #[token("loop")]
    Loop,
    #[token("while")]
    While,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("then")]
    Then,
    #[token("for")]
    For,
    #[token("return")]
    RETURN_KW,
    #[token("continue")]
    Continue,
    #[token("break")]
    Break,
    #[token("match")]
    Match,

    #[token("exposing")]
    Exposing,
    #[token("export")]
    Export,
    #[token("as")]
    As,
    #[token("lib")]
    Library,
    #[token("end")]
    End,
    #[token("pkg")]
    Package,
    #[token("exposed")]
    Exposed,
    #[token("empty")]
    Empty,

    #[token("or")]
    Or,
    #[token("and")]
    And,
    #[token("where")]
    Where,

    #[token("=")]
    Equal,
    #[token("+=")]
    AddAssign,
    #[token("-=")]
    SubAssign,
    #[token("*=")]
    MultAssign,
    #[token("/=")]
    DivAssign,
    #[token("%=")]
    ModAssign,
    #[token("**=")]
    PowAssign,
    #[token("<<=")]
    ShlAssign,
    #[token(">>=")]
    ShrAssign,
    #[token("|=")]
    OrAssign,
    #[token("&=")]
    AndAssign,
    #[token("^=")]
    XorAssign,

    #[token("==")]
    IsEqual,
    #[token("!=")]
    IsNotEqual,
    #[token(">=")]
    GreaterThanEqual,
    #[token("<=")]
    LessThanEqual,
    #[token("<")]
    LeftCaret,
    #[token(">")]
    RightCaret,

    #[token("!")]
    Bang,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("/")]
    Divide,
    #[token("*")]
    Star,
    #[token("%")]
    Modulo,
    #[token("**")]
    DoubleStar,
    #[token("<<")]
    Shl,
    #[token(">>")]
    Shr,
    #[token("|")]
    Pipe,
    #[token("&")]
    Ampersand,
    #[token("^")]
    Caret,

    #[token("[")]
    LeftBrace,
    #[token("]")]
    RightBrace,
    #[token("(")]
    L_PAREN,
    #[token(")")]
    R_PAREN,
    #[token("{")]
    L_BRACE,
    #[token("}")]
    R_BRACE,
    #[token("->")]
    THIN_ARROW,
    #[token("<-")]
    LeftArrow,
    #[token("=>")]
    RightRocket,

    #[token("@")]
    AtSign,
    #[token(",")]
    Comma,
    #[token(";")]
    SEMICOLON,
    #[token(":")]
    Colon,
    #[token(".")]
    Dot,
    #[token("..")]
    DoubleDot,
}

struct TokenStream {
    /// The **lexer** that will be used to lex the input string.
    // lexer: Lexer<'a, Token>,
    tokens: Vec<Token>,
    /// The **current line** of the lexer in the input string.
    /// This is used to provide better error messages.
    line: usize,
    /// The **current offset** of the lexer in the input string.
    offset: usize,
}

// impl TokenStream {
//     // impl<'a> TokenStream<'a> {
//     fn new(input: &str) -> Self {
//         let lexer = Token::lexer(input);

//         let tokens = lexer.collect::<Vec<_>>();

//         Self { tokens, offset: 0 }
//         // Self { lexer: Token::lexer(input), offset: 0 }
//     }

//     // fn peek(&self) -> Option<Token> {
//     //     self.lexer.clone().peekable().peek().as_ref().copied().copied()
//     // }

//     // fn peek_nth(&self, n: usize) -> Option<Token> {
//     //     debug_assert!(n < 3);
//     //     self.lexer.clone().peekable().nth(n).as_ref().copied()
//     // }

//     // fn next(&mut self) -> Option<Token> {
//     //     self.lexer.next()
//     // }

//     // fn has_next(&self) -> bool {
//     //     self.lexer.clone().peekable().peek().is_some()
//     // }

//     // fn expect(&mut self, token: Token) -> Result<(), ParseError> {
//     //     if self.current == Some(&token) {
//     //         self.advance();
//     //         Ok(())
//     //     } else {
//     //         // Err(ParseError::UnexpectedToken { expected: token, found: self.current })
//     //     }
//     // }

//     // fn expect_one_of(&mut self, tokens: &[Token]) -> Result<(), ParseError> {
//     //     if tokens.contains(self.current()) {
//     //         self.advance();
//     //         Ok(())
//     //     } else {
//     //         // Err(ParseError::UnexpectedToken { expected: Token::ERROR, found: self.current })
//     //     }
//     // }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use logos::Lexer;
//     use pretty_assertions_sorted::assert_eq;

//     // #[test]
//     // fn test_lexer() {
//     //     let mut lexer = Lexer::new("fn main() -> int { return 0; }");
//     //     assert_eq!((lexer.next(), lexer.slice(), lexer.span()), (Some(Token::FN_KW), "fn", 0..2));
//     //     assert_eq!(
//     //         (lexer.next(), lexer.slice(), lexer.span()),
//     //         (Some(Token::WHITESPACE), " ", 2..3)
//     //     );
//     //     assert_eq!((lexer.next(), lexer.slice(), lexer.span()), (Some(Token::IDENT), "main", 3..7));
//     //     assert_eq!((lexer.next(), lexer.slice(), lexer.span()), (Some(Token::L_PAREN), "(", 7..8));
//     //     assert_eq!((lexer.next(), lexer.slice(), lexer.span()), (Some(Token::R_PAREN), ")", 8..9));
//     //     assert_eq!(
//     //         (lexer.next(), lexer.slice(), lexer.span()),
//     //         (Some(Token::WHITESPACE), " ", 9..10)
//     //     );
//     //     assert_eq!(
//     //         (lexer.next(), lexer.slice(), lexer.span()),
//     //         (Some(Token::THIN_ARROW), "->", 10..12)
//     //     );
//     //     assert_eq!(
//     //         (lexer.next(), lexer.slice(), lexer.span()),
//     //         (Some(Token::WHITESPACE), " ", 12..13)
//     //     );
//     //     assert_eq!(
//     //         (lexer.next(), lexer.slice(), lexer.span()),
//     //         (Some(Token::IDENT), "int", 13..16)
//     //     );
//     //     assert_eq!(
//     //         (lexer.next(), lexer.slice(), lexer.span()),
//     //         (Some(Token::WHITESPACE), " ", 16..17)
//     //     );
//     //     assert_eq!(
//     //         (lexer.next(), lexer.slice(), lexer.span()),
//     //         (Some(Token::L_BRACE), "{", 17..18)
//     //     );
//     //     assert_eq!(
//     //         (lexer.next(), lexer.slice(), lexer.span()),
//     //         (Some(Token::WHITESPACE), " ", 18..19)
//     //     );
//     //     assert_eq!(
//     //         (lexer.next(), lexer.slice(), lexer.span()),
//     //         (Some(Token::RETURN_KW), "return", 19..25)
//     //     );
//     //     assert_eq!(
//     //         (lexer.next(), lexer.slice(), lexer.span()),
//     //         (Some(Token::WHITESPACE), " ", 25..26)
//     //     );
//     //     assert_eq!(
//     //         (lexer.next(), lexer.slice(), lexer.span()),
//     //         (Some(Token::INTEGER), "0", 26..27)
//     //     );
//     //     assert_eq!(
//     //         (lexer.next(), lexer.slice(), lexer.span()),
//     //         (Some(Token::SEMICOLON), ";", 27..28)
//     //     );
//     //     assert_eq!(
//     //         (lexer.next(), lexer.slice(), lexer.span()),
//     //         (Some(Token::WHITESPACE), " ", 28..29)
//     //     );
//     //     assert_eq!(
//     //         (lexer.next(), lexer.slice(), lexer.span()),
//     //         (Some(Token::R_BRACE), "}", 29..30)
//     //     );
//     //     assert_eq!((lexer.next(), lexer.slice(), lexer.span()), (None, "", 30..30));
//     // }

//     // #[test]
//     // fn test_token_stream() {
//     //     let mut token_stream = TokenStream::new("fn main() -> int { return 0; }");

//     //     assert_eq!(token_stream.peek(), Some(Token::FN_KW));
//     //     assert_eq!(token_stream.next(), Some(Token::FN_KW));
//     // }
// }
