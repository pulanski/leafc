/// Contains the **location** of an **item** in a **file**.
pub mod file;

/// Contains the **span** of an **item** in a **file** (_i.e. `17..20`_).
pub mod span;

/// Contains information about a **text slice**.
pub mod text;

use getset::{
    MutGetters,
    Setters,
};
use leafc_intern::string::StringInterner;

use self::file::File;

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
#[derive(Debug, Clone, MutGetters, Setters, PartialEq)]
#[getset(get_mut = "pub", set = "pub")]
pub struct Location {
    /// A **file** that contains the **item**.
    file: File,

    /// The **span** of the location in the file.
    span: Span,

    // span: Span,
    /// The **string interner** that is used to **intern** the **text** of the
    /// **item**.
    interner: StringInterner,
}

impl Location {
    /// Returns the **file id** of the **file** that contains the **item**.
    /// This is used to **lookup** the **file** in the [`FileSet`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leafc_utils::codemap::Location;
    /// use leafc_utils::file::FileId;
    ///
    /// let file_id = FileId::new(42);
    /// let location = Location::new(file_id, 17..20, StringInterner::new());
    /// assert_eq!(location.file_id(), file_id);
    /// ```
    #[inline]
    #[must_use]
    pub const fn file_id(&self) -> FileId {
        self.file.id()
    }
}

// pub struct CodemapContext {
//     file_set:        FileSet,
//     string_interner: StringInterner,
//     // span_interner: SpanInterner, where SpanInterner = Interner<Span>,
//     // file_interner: StringInterner,
// }

// impl From<Span> for LineColumn {
//     /// Converts the given `span` (e.g. `17..20`) into a `LineColumn` (e.g.
//     /// `LineColumn { line: 2, column: 3 }`). This is used to **convert** a
//     /// `Span` into a `LineColumn` for **displaying** the **location** of an
//     /// **item** in a **file** (_e.g. `2:3`_).
//     ///
//     /// # Example
//     fn from(span: Span) -> Self {
//         let line = 0;
//         let offset = 0;

//         // let line = span.start().line();
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
    pub fn new<F: Into<File>, S: Into<Span>, I: Into<StringInterner>>(
        file: F,
        span: S,
        interner: I,
    ) -> Self {
        Self { file: file.into(), span: span.into(), interner: interner.into() }
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

    /// Returns the **line** and **column** of the **start** of the **span**.
    /// The **line** and **column** are **one-indexed**.
    /// The **column** is the **offset** from the **start of the line**.
    /// The **line** is the **offset** from the **start of the file**.
    ///
    /// # Example
    ///
    /// ```rust
    /// use leafc_utils::codemap::{
    ///     FileId,
    ///     Location,
    ///     Span,
    /// };
    ///
    /// let location = Location::new(FileId::new(0), Span::from(0..10));
    ///
    /// assert_eq!(location.get_line_column(), (1, 1));
    /// ```
    #[must_use]
    #[inline]
    #[allow(clippy::missing_panics_doc)]
    pub fn get_line_column(&self) -> LineColumn {
        // Get the line and column of the start of the span in the file.

        let file = self.file();

        // TODO: refactor to
        // let interner = self.interner(); where self.interner() =
        // self.file().str_interner();

        // let source_text = interner.get_source_text();

        let interner = file.str_interner();
        let source_text = file.source_text(interner);

        //     let line = self.span_start().line();
        //     let column = self.span_start().column();

        //     (line, column)
        todo!()
    }

    /// Returns the **span** of the **location**.
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
    /// assert_eq!(location.span(), Span::from(0..10));
    /// ```
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Returns the **file** of the **location**.
    /// The **file** is the **file** that contains the **item**.
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
    /// assert_eq!(location.file(), FileId::new(0));
    /// ```
    #[must_use]
    pub fn file(&self) -> File {
        self.file.clone()
    }
}

#[cfg(test)]
mod location_test_suite {
    use super::*;
    use pretty_assertions_sorted::assert_eq;

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
        let string_interner = StringInterner::default();
        let file = File::new(FileId::new(1), "test.rs", "test");

        let location = Location::new(file, Span::from(0..1), string_interner);

        assert_eq!(
            (location.span_start(), location.span_end()),
            (TextPosition::from(0), TextPosition::from(1))
        );
        assert_eq!(location.file_id(), FileId::from(1));

        // let interner = StringInterner::default();
        // let file = File::new(FileId::new(0), "test.rs", "test");
        // // let location = Location::new(file_id, span, interner);

        // assert_eq!(
        //     (location.span_start(), location.span_end()),
        //     (TextPosition::from(0), TextPosition::from(1))
        // );
        // assert_eq!(location.file(), file_id);
    }
}
