#![allow(dead_code)]
//!

use owo_colors::OwoColorize;
use std::fmt;

use getset::{Getters, MutGetters, Setters};

/// A **range** of text in the input string (_i.e. a **span**_).
///
/// It is used to provide **context** for error messages and
/// to **highlight** the source code.
/// It is also used to **track** the **location** of the source code.
///
/// **NOTE:** The **span** maintains an **invariant** that the **start** of the span
/// is **less than or equal to** the **end** of the span (_i.e. `start <= end`_).
///
/// # Example
///
/// ```
/// use lexer::Token;
///
/// let input = "let x = 5;";
/// let lexer = Token::lexer(input);
///
/// let tokens = lexer.collect::<Vec<_>>();
///
/// assert_eq!(tokens[0].span().start().line(), 1);
/// assert_eq!(tokens[0].span().start().offset(), 0);
/// assert_eq!(tokens[0].span().end().line(), 1);
/// assert_eq!(tokens[0].span().end().offset(), 3);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Getters, MutGetters, Setters)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct Span {
    /// The **start** of the span.
    start: usize,
    /// The **end** of the span.
    end: usize,
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.start.magenta().italic(),
            "..".black().italic(),
            self.end.magenta().italic()
        )
    }
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        debug_assert!(start <= end);

        Self { start, end }
    }
}

/// Used to store information about the **location** of an **item in a file**.
/// An **item** can be a **token**, a **node**, or a **span**.
pub struct Location {
    /// A **unique identifier** for the **file** that the location is in.
    file: FileId,

    /// The **span** of the location in the file.
    span: Span,
}

/// A **position** in the input string (e.g. a **line** and an **offset**
/// in the line).
///
/// This is used to provide **context** for error messages and
/// to **highlight** the source code.
///
/// # Example
///
/// ```
/// // TODO: add example
/// ```
pub struct FilePosition {
    /// The **line** of the position.
    line: usize,
    /// The **offset** of the position.
    offset: usize,
}

/// A **unique identifier** for a **file**.
pub struct FileId(usize);
