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
    /// Whitespace is captured primarily for the purpose of constructing
    /// **high-fidelity, lossless syntax trees**. This idea is by no means
    /// new, as is in large part inspired by its implementations in systems
    /// such as [**rust-analyzer**]() (_rowan_), the [**Swift compiler**]()
    /// (_libsyntax_), and in [**C# compiler**]() (_Roslyn_).
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
    /// comments can be prefixed by either `//` or '#' and multi-line comments
    /// are prefixed with `/*` and suffixed with `*/`.
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
