use std::{
    fmt,
    ops::{
        Range,
        RangeInclusive,
    },
};

use getset::{
    CopyGetters,
    MutGetters,
    Setters,
};
use owo_colors::OwoColorize;

use super::text::TextPosition;

/// A **range** of text in the input string (_i.e. a **span**_).
///
/// It is used to provide **context** for error messages and
/// to **highlight** the source code.
/// It is also used to **track** the **location** of the source code.
///
/// **NOTE:** The **span** maintains an **invariant** that the **start** of the
/// span is **less than or equal to** the **end** of the span (_i.e. `start <=
/// end`_).
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
#[getset(get_copy = "pub", get_mut = "pub")]
pub struct Span {
    /// The **start** of the span.
    ///
    /// # Example
    ///
    /// ```rust
    /// use leafc_utils::codemap::Span;
    ///
    /// let span = Span::new(0, 10);
    ///
    /// assert_eq!(0, span.start());
    /// ```
    start: TextPosition,

    /// The **end** of the span.
    ///
    /// # Example
    ///
    /// ```rust
    /// use leafc_utils::codemap::Span;
    ///
    /// let span = Span::new(0, 10);
    ///
    /// assert_eq!(10, span.end());
    /// ```
    end: TextPosition,
}

#[allow(clippy::from_over_into)]
impl Into<Range<usize>> for Span {
    /// Converts a `Span` to a `Range<usize>`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use leafc_utils::Span;
    ///
    /// let span = Span::new(0, 10);
    /// let range = span.into();
    ///
    /// assert_eq!(range, 0..10);
    /// ```
    fn into(self) -> Range<usize> {
        self.start().into()..self.end().into()
    }
}

impl From<Range<usize>> for Span {
    /// Converts a `Range<usize>` to a `Span`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use leafc_utils::Span;
    ///
    /// let range = 0..10;
    /// let span = range.into();
    ///
    /// assert_eq!(span, Span::new(0, 10));
    /// ```
    fn from(range: Range<usize>) -> Self {
        Self::new(range.start..range.end)
    }
}

impl From<RangeInclusive<usize>> for Span {
    /// Converts a `RangeInclusive<usize>` to a `Span`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use leafc_utils::Span;
    ///
    /// assert_eq!(Span::new(0..10), Span::from(0..=9));
    /// ```
    #[allow(clippy::range_plus_one)] // RangeInclusive and Range are different data types
    fn from(range: RangeInclusive<usize>) -> Self {
        let (range_start, range_end) = range.into_inner();
        Self::new(range_start..range_end + 1)
    }
}

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
    /// use leafc_utils::codemap::Span;
    ///
    /// let span = Span::new(0..10);
    ///
    /// assert_eq!(0, span.start());
    /// assert_eq!(10, span.end());
    ///
    /// let span = Span::new(0..0);
    ///
    /// assert_eq!(0, span.start());
    /// assert_eq!(0, span.end());
    ///
    /// // This will panic in debug builds
    ///
    /// // let span = Span::new(10..0);
    /// ```
    #[must_use]
    pub fn new<T: Into<TextPosition>>(Range { start, end }: Range<T>) -> Self {
        let (start, end) = (start.into(), end.into());
        debug_assert!(start <= end, "start must be <= end");

        Self { start, end }
    }

    // #[must_use]
    // pub fn new<T: Into<TextPosition>>(start: T, end: T) -> Self {
    //     let (start, end) = (start.into(), end.into());
    //     debug_assert!(start <= end, "start must be <= end");

    //     Self { start, end }
    // }

    /// Sets the **start** of the span.
    ///
    /// # Panics
    ///
    /// Panics if the `start` is **greater than** the `end` (NOTE: this
    /// only occurs in **debug** builds)
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::Span;
    ///
    /// let mut span = Span::new(0, 10);
    ///
    /// assert_eq!(0, span.start());
    ///
    /// span.set_start(5);
    ///
    /// assert_eq!(5, span.start());
    ///
    /// // This will panic in debug builds
    /// //
    /// // span.set_start(15);
    /// ```
    pub fn set_start<T: Into<TextPosition>>(&mut self, start: T) {
        let start = &start.into();
        debug_assert!(*start <= self.end);

        self.start = *start;
    }

    /// Sets the **end** of the span.
    ///
    /// # Panics
    ///
    /// Panics if the `end` is **less than** the `start` (NOTE: this
    /// only occurs in **debug** builds)
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::Span;
    ///
    /// let mut span = Span::new(0, 10);
    ///
    /// assert_eq!(10, span.end());
    ///
    /// span.set_end(15);
    ///
    /// assert_eq!(15, span.end());
    ///
    /// // This will panic in debug builds
    /// //
    /// // span.set_end(5);
    /// ```
    pub fn set_end<T: Into<TextPosition>>(&mut self, end: T) {
        let end = &end.into();
        debug_assert!(self.start <= *end);

        self.end = *end;
    }

    /// Returns the **length** of the span.
    /// This is equivalent to `end - start`.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::Span;
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
    pub fn len(&self) -> usize {
        (self.end - self.start).into()
    }

    /// Returns `true` if the span is empty.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::Span;
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
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns `true` if the given `other` span is contained within the span.
    /// This is **inclusive** of the **start** and **end** of the span.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::Span;
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
    pub fn contains(&self, other: &Self) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }

    /// Returns `true` if the given `index` is contained within the span.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::Span;
    ///
    /// let span = Span::new(0, 10);
    ///
    /// assert!(span.contains_index(5));
    /// assert!(!span.contains_index(15));
    /// ```
    #[must_use]
    pub fn contains_index(&self, index: usize) -> bool {
        let index = TextPosition::from(index);

        self.start <= index && self.end >= index
    }

    /// Returns the **range** of the span.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::Span;
    ///
    /// let span = Span::new(0, 10);
    ///
    /// assert_eq!(0..10, span.range());
    /// ```
    #[must_use]
    pub fn range(&self) -> Range<TextPosition> {
        self.start()..self.end()
    }

    /// Merge the given `other` span into the span.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::Span;
    ///
    /// let mut span = Span::new(0, 10);
    ///
    /// span.merge(&Span::new(5, 15));
    ///
    /// assert_eq!(0..15, span.range());
    /// ```
    pub fn merge(&mut self, other: &Self) {
        self.start = self.start().min(other.start);
        self.end = self.end().max(other.end);
    }
}

#[cfg(test)]
mod span_test_suite {
    use super::*;

    #[test]
    fn test_span_new() {
        let span = Span::new(0..10);

        assert_eq!(TextPosition::new(0), span.start());
        assert_eq!(TextPosition::new(10), span.end());
    }

    #[test]
    fn test_span_len() {
        let span = Span::new(0..10);

        assert_eq!(10, span.len());
    }

    #[test]
    fn test_span_is_empty() {
        let span = Span::new(0..10);

        assert!(!span.is_empty());

        let span = Span::new(0..0);

        assert!(span.is_empty());
    }

    #[test]
    fn test_span_contains() {
        let span = Span::new(0..10);

        assert!(span.contains(&Span::new(0..10)));
        assert!(span.contains(&Span::new(0..5)));

        assert!(!span.contains(&Span::new(0..15)));
        assert!(!span.contains(&Span::new(5..15)));
    }

    #[test]
    fn test_span_contains_index() {
        let span = Span::new(0..10);

        assert!(span.contains_index(5));
        assert!(!span.contains_index(15));
    }

    #[test]
    fn test_span_range() {
        let span = Span::new(0..10);

        assert_eq!(TextPosition::new(0)..TextPosition::new(10), span.range());
    }

    #[test]
    fn test_span_merge() {
        let mut span = Span::new(0..10);

        span.merge(&Span::new(5..15));

        assert_eq!(TextPosition::new(0)..TextPosition::new(15), span.range());
    }

    #[test]
    fn test_span_from() {
        let span = Span::from(0..10);

        assert_eq!(TextPosition::from(0), span.start());
        assert_eq!(TextPosition::from(10), span.end());

        let span = Span::from(0..=10);

        assert_eq!(TextPosition::from(0), span.start());
        assert_eq!(TextPosition::from(11), span.end());
    }
}

/// A **spanned** value.
/// This is a value with a span.
/// This is useful for storing the location of a value of an
/// **arbitrary** type `T` within a source file.
///
/// # Example
///
/// ```
/// use leafc_utils::codemap::{
///     Span,
///     Spanned,
/// };
///
/// // Create a spanned value, with a span of `0..12`.
/// let spanned = Spanned::new("Hello, World!", Span::new(0, 12));
///
/// // Get the value.
/// assert_eq!("Hello, World!", spanned.value());
/// // Get the span.
/// assert_eq!(0..12, spanned.span_range());
///
/// // Set the value.
/// spanned.set_value("Hello, Universe!");
///
/// // The span is unchanged, but the value is.
/// assert_eq!("Hello, Universe!", spanned.value());
/// assert_eq!(0..12, spanned.span_range());
///
/// // Set the span.
/// spanned.set_span(Span::from(5, 15));
///
/// assert_eq!(5, spanned.span_start());
/// assert_eq!(15, spanned.span_end());
///
/// // The value is unchanged, but the span is.
/// assert_eq!(16, spanned.span_len());
///
/// // Additional APIs ...
///
/// spanned.set_span_start(5);
/// spanned.set_span_end(15);
///
/// assert_eq!(5..15, spanned.span_range());
/// ```
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, CopyGetters, MutGetters, Setters,
)]
pub struct Spanned<T> {
    /// The value.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::{
    ///     Span,
    ///     Spanned,
    /// };
    ///
    /// let spanned = Spanned::new(5, Span::new(0, 10));
    ///
    /// assert_eq!(5, spanned.value());
    /// ```
    pub value: T,

    /// The span of the value.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::{
    ///     Span,
    ///     Spanned,
    /// };
    ///
    /// let spanned = Spanned::new(5, Span::new(0, 10));
    ///
    /// assert_eq!(Span::new(0, 10), spanned.span());
    /// ```
    pub span: Span,
}

impl<T> Spanned<T> {
    /// Creates a new **spanned** value.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::{
    ///     Span,
    ///     Spanned,
    /// };
    ///
    /// let spanned = Spanned::new(5, Span::new(0, 10));
    ///
    /// assert_eq!(Span::new(0, 10), spanned.span);
    /// assert_eq!(5, spanned.value);
    /// ```
    #[must_use]
    pub const fn new(value: T, span: Span) -> Self {
        Self { value, span }
    }

    /// Returns the **start** of the span.
    ///
    /// _This is the same as `self.span.start()`._
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::{
    ///     Span,
    ///     Spanned,
    /// };
    ///
    /// let spanned = Spanned::new(5, Span::new(0, 10));
    ///
    /// assert_eq!(0, spanned.start());
    /// ```
    #[must_use]
    pub fn start(&self) -> TextPosition {
        self.span.start()
    }

    /// Returns the **end** of the span.
    ///
    /// _This is the same as `self.span.end()`._
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::{
    ///     Span,
    ///     Spanned,
    /// };
    ///
    /// let spanned = Spanned::new(Span::new(0, 10), 5);
    ///
    /// assert_eq!(10, spanned.end());
    /// ```
    #[must_use]
    pub fn end(&self) -> TextPosition {
        self.span.end()
    }

    /// Returns the **length** of the span.
    ///
    /// _This is the same as `self.span.len()`._
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::{
    ///     Span,
    ///     Spanned,
    /// };
    ///
    /// let spanned = Spanned::new(Span::new(0, 10), 5);
    ///
    /// assert_eq!(10, spanned.span_len());
    /// ```
    #[must_use]
    pub fn span_len(&self) -> usize {
        self.span.len()
    }

    /// Returns `true` if the span is empty.
    ///
    /// _This is the same as `self.span.is_empty()`._
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::{
    ///     Span,
    ///     Spanned,
    /// };
    ///
    /// let spanned = Spanned::new(Span::new(0, 10), 5);
    ///
    /// assert!(!spanned.span_is_empty());
    /// ```
    #[must_use]
    pub fn span_is_empty(&self) -> bool {
        self.span.is_empty()
    }

    /// Applies the function `f` to the contained value (if any), and
    /// returns a new value with the **same span**.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::{
    ///     Span,
    ///     Spanned,
    /// };
    ///
    /// let spanned = Spanned::new(Span::new(0, 10), 5);
    ///
    /// let spanned = spanned.map(|value| value + 1);
    ///
    /// assert_eq!(6, spanned.value);
    /// ```
    pub fn into_map<U>(self, f: impl FnOnce(T) -> U) -> Spanned<U> {
        Spanned::new(f(self.value), self.span)
    }

    /// Applies the function `f` to the contained value (if any), and
    /// returns a new value with the **same span**.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::{
    ///     Span,
    ///     Spanned,
    /// };
    ///
    /// let spanned = Spanned::new(Span::new(0, 10), 5);
    ///
    /// let spanned = spanned.map(|value| value + 1);
    ///
    /// assert_eq!(6, spanned.value);
    /// ```
    pub fn map<U>(&self, f: impl FnOnce(&T) -> U) -> Spanned<U> {
        Spanned::new(f(&self.value), self.span)
    }
}
