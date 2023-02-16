use derive_new::new;
use owo_colors::OwoColorize;
use std::{fmt, ops::Range};

use getset::{CopyGetters, MutGetters, Setters};

/// A **unique identifier** for a **file**.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, new)]
pub struct FileId(usize);

// pub struct File {
//     id: FileId,
//     abs_path: String,
//     text: String,
// }

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
/// ```rust,ignore
/// // TODO: update this example
/// use leafc_lexer::TokenStream;
///
/// let input = "x := 5;";
/// let lexer = TokenStream::new(input);
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, CopyGetters, MutGetters, Setters)]
#[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
pub struct Span {
    /// The **start** of the span.
    start: usize,
    /// The **end** of the span.
    end: usize,
}

/// Used to store information about the **location** of an **item in a file**.
/// An **item** can be a **token**, a **node**, or a **span**.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, CopyGetters, MutGetters, Setters)]
#[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
pub struct Location {
    /// A **unique identifier** for the **file** that the location is in.
    file: FileId,

    /// The **span** of the location in the file.
    span: Span,
}

/// A **wrapper** around an **item** that also stores its **location**.
/// An **item** can be a **token**, a **node**, or a **span**.
///
/// # Example
///
/// ```ignore
/// // TODO: add example
/// use leafc_lexer::TokenStream;
/// use leafc_utils::Locatable;
/// use leafc_lexer::Token;
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, CopyGetters, MutGetters, Setters)]
#[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
pub struct Locatable<T: Clone + Copy> {
    /// The **location** of the item.
    location: Location,
    /// The **item** itself.
    item: T,
}

impl<T: Clone + Copy> Locatable<T> {
    /// Creates a new `Locatable` from the given `location` and `item`.
    ///
    /// # Example
    ///
    /// ```
    /// // TODO: add example
    /// use leafc_utils::{Locatable, Location, FileId, Span};
    ///
    /// let location = Location::new(FileId::new(0), Span::new(0, 1));
    /// ```
    pub const fn new(location: Location, item: T) -> Self {
        Self { location, item }
    }

    /// Returns the **span** of the `Locatable` item.
    ///
    /// # Example
    ///
    /// ```
    /// // TODO: add example
    /// use leafc_utils::Locatable;
    /// ```
    pub fn span(&self) -> Span {
        self.location.span()
    }

    /// Returns the **file** (i.e. the `FileId`) of the `Locatable` item.
    ///
    /// # Example
    ///
    /// ```
    /// // TODO: add example
    /// use leafc_utils::Locatable;
    /// ```
    pub fn file(&self) -> FileId {
        self.location.file()
    }

    /// Returns the **range** of the `Locatable` item.
    /// The **range** is the **span** of the `Locatable` item, more specifically
    /// a **tuple** of the **start** and **end** of the `Locatable` item.
    ///
    /// # Example
    ///
    /// ```
    /// // TODO: add example
    /// use leafc_utils::Locatable;
    /// ```
    pub fn range(&self) -> Range<usize> {
        self.location.span().range()
    }

    /// Returns the **start** of the `Locatable` item.
    ///
    /// # Example
    ///
    /// ```
    /// // TODO: add example
    /// use leafc_utils::Locatable;
    /// ```
    pub fn start(&self) -> usize {
        self.location.span().start()
    }

    /// Returns the **end** of the `Locatable` item.
    ///
    /// # Example
    ///
    /// ```
    /// // TODO: add example
    /// use leafc_utils::Locatable;
    /// ```
    pub fn end(&self) -> usize {
        self.location.span().end()
    }

    // pub fn map<U: Clone, F: FnOnce(T) -> U>(self, f: F) -> Locatable<U> {
    //     Locatable::new(self.location, f(self.item))
    // }
}

/// A **position** in the input string (e.g. a **line** and **offset**
/// into the line).
///
/// This is used to provide **context** for error messages and
/// to **highlight** the source code.
///
/// # Example
///
/// ```ignore
/// // TODO: add example
/// use leafc_lexer::TokenStream;
///
/// let input = "x := 5;";
/// let lexer = TokenStream::new(input);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, CopyGetters, MutGetters, Setters, new)]
pub struct FilePosition {
    /// The **line** of the position.
    line: usize,
    /// The **offset** of the position.
    offset: usize,
}

impl From<(usize, usize)> for FilePosition {
    /// Converts a **tuple** of the **line** and **offset** to a `FilePosition`.
    /// The **tuple** is of the form `(line, offset)`.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::FilePosition;
    ///
    /// let (line, offset) = (0, 0);
    /// let file_position = FilePosition::from((line, offset));
    /// ```
    fn from((line, offset): (usize, usize)) -> Self {
        Self { line, offset }
    }
}

impl From<FilePosition> for (usize, usize) {
    /// Converts a `FilePosition` to a **tuple** of the **line** and **offset**.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::FilePosition;
    ///
    /// let file_position = FilePosition::new(0, 0);
    /// let (line, offset) = file_position.into();
    /// ```
    fn from(position: FilePosition) -> Self {
        (position.line, position.offset)
    }
}

// impl From<Span> for FilePosition {
// todo convert span (start, end) to (line, offset)
//     fn from(span: Span) -> Self {
//          let line = 0;
//          let offset = 0;
//         Self {
//     }
// }

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
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
    /// Creates a new `Span` from the given `start` and `end` indices.
    /// Maintains the invariant that the **start** of the span must be
    /// **less than or equal to** the **end** of the span.
    ///
    /// # Panics
    ///
    /// Panics if the `start` is **greater than** the `end` (NOTE: this
    /// only occurs in **debug** builds)
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::location::Span;
    ///
    /// let span = Span::new(0, 10);
    ///
    /// assert_eq!(0, span.start());
    ///
    /// assert_eq!(10, span.end());
    ///
    /// let span = Span::new(0, 0);
    ///
    /// assert_eq!(0, span.start());
    ///
    /// assert_eq!(0, span.end());
    ///
    /// // This will panic in debug builds
    ///
    /// // let span = Span::new(10, 0);
    /// ```
    #[must_use]
    pub fn new(start: usize, end: usize) -> Self {
        debug_assert!(start <= end);

        Self { start, end }
    }

    /// Returns the **length** of the span.
    /// This is equivalent to `end - start`.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::location::Span;
    ///
    /// let span = Span::new(0, 10);
    ///
    /// assert_eq!(10, span.len());
    ///
    /// let span = Span::new(0, 0);
    ///
    /// assert_eq!(0, span.len());
    /// ```
    #[must_use]
    pub const fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns `true` if the span is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::location::Span;
    ///
    /// let span = Span::new(0, 10);
    ///
    /// assert!(!span.is_empty());
    ///
    /// let span = Span::new(0, 0);
    ///
    /// assert!(span.is_empty());
    /// ```
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns `true` if the given `other` span is contained within the span.
    /// This is **inclusive** of the **start** and **end** of the span.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::location::Span;
    ///
    /// let span = Span::new(0, 10);
    ///
    /// assert!(span.contains(&Span::new(0, 10)));
    /// assert!(span.contains(&Span::new(0, 5)));
    ///
    /// assert!(!span.contains(&Span::new(0, 15)));
    /// assert!(!span.contains(&Span::new(5, 15)));
    /// ```
    #[must_use]
    pub const fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    /// Returns `true` if the given `index` is contained within the span.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::location::Span;
    ///
    /// let span = Span::new(0, 10);
    ///
    /// assert!(span.contains_index(5));
    /// assert!(!span.contains_index(15));
    /// ```
    #[must_use]
    pub const fn contains_index(&self, index: usize) -> bool {
        self.start <= index && self.end >= index
    }

    /// Returns the **range** of the span.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::location::Span;
    ///
    /// let span = Span::new(0, 10);
    ///
    /// assert_eq!(0..10, span.range());
    /// ```
    #[must_use]
    pub const fn range(&self) -> Range<usize> {
        self.start..self.end
    }
}

impl Location {
    /// Creates a new `Location` from the given `span` and `file`.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::location::{Location, Span, FileId};
    ///
    /// let location = Location::new(FileId::new(0), Span::new(0, 10));
    ///
    /// assert_eq!(0, location.span().start());
    /// assert_eq!(10, location.span().end());
    /// ```
    pub fn new<F: Into<FileId>, S: Into<Span>>(file: F, span: S) -> Self {
        Self { file: file.into(), span: span.into() }
    }
}

#[cfg(test)]
mod location_test_suite {
    use super::*;

    #[test]
    #[allow(unused_results)]
    fn test_span_new() {
        let mut span = Span::new(0, 1);

        assert_eq!(span.start(), 0);
        assert_eq!(span.end(), 1);

        span.set_start(1);
        span.set_end(2);

        assert_eq!(span.start(), 1);
        assert_eq!(span.end(), 2);
    }

    #[test]
    #[should_panic = "assertion failed: start <= end"]
    #[allow(unused_must_use)]
    fn test_span_new_panic() {
        Span::new(1, 0);
    }

    #[test]
    fn test_location_new() {
        let location = Location::new(FileId::new(0), Span::new(0, 1));

        assert_eq!(location.span().start(), 0);
        assert_eq!(location.span().end(), 1);
        assert_eq!(location.file(), FileId(0));
    }

    #[test]
    fn test_file_id_new() {
        let file_id = FileId::new(0);

        assert_eq!(file_id.0, 0);
    }
}
