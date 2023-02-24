/// Contains the **location** of an **item** in a **file**.
pub mod file;

/// Contains the **span** of an **item** in a **file** (_i.e. `17..20`_).
pub mod span;

/// Contains information about a **text slice**.
pub mod text;

/// TODO: document this
pub mod cursor;

use getset::{
    CopyGetters,
    MutGetters,
    Setters,
};

pub use {
    file::{
        FileData,
        FileId,
        FileSet,
        LineColumn,
    },
    span::{
        Span,
        Spanned,
    },
    text::TextPosition,
};

// pub enum Item {
//     Module,
// }

/// Used to store information about the **location** of an **item in a file**.
/// An **item** can be a **token**, a **node**, a
/// **statement**, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, CopyGetters, MutGetters, Setters)]
#[getset(get_copy = "pub", get_mut = "pub", set = "pub")]
pub struct Location {
    /// A **unique identifier** for the **file** that the location is in.
    pub file: FileId,

    /// The **span** of the location in the file.
    span: Span,
}

// impl From<Span> for LineColumn {
// todo convert span (start, end) to (line, offset)
//     fn from(span: Span) -> Self {
//          let line = 0;
//          let offset = 0;
//         Self {
//     }
// }

impl Location {
    /// Creates a new `Location` from the given `span` and `file`.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::{
    ///     FileId,
    ///     Location,
    ///     Span,
    /// };
    ///
    /// let location = Location::new(FileId::new(0), Span::from(0..10));
    ///
    /// assert_eq!(0, location.span_start());
    /// assert_eq!(10, location.span_end());
    /// ```
    pub fn new<F: Into<FileId>, S: Into<Span>>(file: F, span: S) -> Self {
        Self { file: file.into(), span: span.into() }
    }

    /// Returns the **start** of the **span**.
    ///
    /// _This is the same as `self.span().start()`._
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::location::{
    ///     FileId,
    ///     Location,
    ///     Span,
    /// };
    ///
    /// let location = Location::new(FileId::new(0), Span::from(0..10));
    ///
    /// assert_eq!(0, location.span_start());
    /// ```
    #[must_use]
    pub fn span_start(&self) -> TextPosition {
        self.span().start()
    }

    /// Returns the **end** of the **span**.
    ///
    /// _This is the same as `self.span().end()`._
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::location::{
    ///     FileId,
    ///     Location,
    ///     Span,
    /// };
    ///
    /// let location = Location::new(FileId::new(0), Span::from(0..10));
    ///
    /// assert_eq!(10, location.span_end());
    /// ```
    #[must_use]
    pub fn span_end(&self) -> TextPosition {
        self.span().end()
    }
}

#[cfg(test)]
mod location_test_suite {
    use super::*;

    #[test]
    #[allow(unused_results)]
    fn test_span_new() {
        let mut span = Span::from(0..1);

        assert_eq!(span.start(), TextPosition::from(0));
        assert_eq!(span.end(), TextPosition::from(1));

        span.set_start(1);
        span.set_end(2);

        assert_eq!(span.start(), TextPosition::from(1));
        assert_eq!(span.end(), TextPosition::from(2));
    }

    #[test]
    #[should_panic = "start must be <= end"]
    #[allow(unused_must_use, clippy::reversed_empty_ranges)]
    fn test_span_new_panic() {
        Span::new(1..0);
    }

    #[test]
    fn test_location_new() {
        let (file_id, span) = (FileId::new(), Span::from(0..1));
        let location = Location::new(file_id, span);

        assert_eq!(
            (location.span_start(), location.span_end()),
            (TextPosition::from(0), TextPosition::from(1))
        );
        assert_eq!(location.file(), file_id);
    }
}
