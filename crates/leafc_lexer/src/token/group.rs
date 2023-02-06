use std::fmt;

use smol_str::SmolStr;

use super::TokenKind;

// impl TokenKind {
//     #[allow(clippy::wrong_self_convention)]
//     fn to_string(&self) -> SmolStr {
//         let str = match self {
//             Self::ERROR => "Error",

//             Self::IDENT => "Ident",

//             Self::COMMENT => "Comment",
//             Self::DocComment => "DocComment",
//             Self::WHITESPACE => " ",

//             Self::String => "str",
//             Self::Rune => "rune",
//             Self::INTEGER => "int",
//             Self::Float => "float",
//             Self::Bool => "bool",

//             Self::Let => "let",
//             Self::RETURN_KW => "return",
//             Self::Continue => "continue",
//             Self::Break => "break",
//             Self::Package => "pkg",
//             Self::Exposed => "exposed",
//             Self::Empty => "empty",
//             Self::Then => "then",
//             Self::For => "for",
//             Self::Or => "or",
//             Self::And => "and",
//             Self::Type => "type",
//             Self::Loop => "loop",
//             Self::While => "while",
//             Self::If => "if",
//             Self::Else => "else",
//             Self::FN_KW => "fn",
//             Self::Enum => "enum",
//             Self::Trait => "trait",
//             Self::Import => "import",
//             Self::Exposing => "exposing",
//             Self::Export => "export",
//             Self::As => "as",
//             Self::Library => "lib",
//             Self::End => "end",
//             Self::In => "in",
//             Self::Is => "is",
//             Self::Match => "match",
//             Self::Where => "where",
//             Self::Const => "comptime",
//             Self::Extend => "extend",
//             Self::With => "with",
//             Self::Alias => "alias",
//             Self::Mut => "mut",
//             Self::Ref => "ref",
//             Self::Extern => "extern",
//             Self::Unsafe => "unsafe",

//             Self::Equal => "=",
//             Self::AddAssign => "+=",
//             Self::SubAssign => "-=",
//             Self::MultAssign => "*=",
//             Self::DivAssign => "/=",
//             Self::ModAssign => "%=",
//             Self::PowAssign => "**=",
//             Self::ShlAssign => "<<=",
//             Self::ShrAssign => ">>=",
//             Self::OrAssign => "|=",
//             Self::AndAssign => "&=",
//             Self::XorAssign => "^=",

//             Self::IsEqual => "==",
//             Self::IsNotEqual => "!=",
//             Self::GreaterThanEqual => ">=",
//             Self::LessThanEqual => "<=",
//             Self::LeftCaret => "<",
//             Self::RightCaret => ">",

//             Self::Bang => "!",

//             Self::Plus => "+",
//             Self::Minus => "-",
//             Self::Divide => "/",
//             Self::Star => "*",
//             Self::Modulo => "%",
//             Self::DoubleStar => "**",
//             Self::Pipe => "|",
//             Self::Caret => "^",
//             Self::Ampersand => "&",
//             Self::Shl => "<<",
//             Self::Shr => ">>",

//             Self::LeftBrace => "[",
//             Self::RightBrace => "]",
//             Self::L_PAREN => "(",
//             Self::R_PAREN => ")",
//             Self::L_BRACE => "{",
//             Self::R_BRACE => "}",
//             Self::THIN_ARROW => "->",
//             Self::LeftArrow => "<-",
//             Self::RightRocket => "=>",

//             Self::AtSign => "@",
//             Self::Comma => ",",
//             Self::SEMICOLON => ";",
//             Self::Colon => ":",
//             Self::Dot => ".",
//             Self::DoubleDot => "..",
//         };

//         str.into()
//     }
// }

// impl fmt::Display for TokenKind {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.write_str(self.to_string().as_str())
//     }
// }

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
