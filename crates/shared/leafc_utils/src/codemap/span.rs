use std::{
    fmt,
    ops::{
        Deref,
        DerefMut,
        Range,
        RangeInclusive,
    },
};

use dupe::Dupe;
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
#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, CopyGetters, MutGetters, Setters, Dupe,
)]
#[getset(get_copy = "pub", get_mut = "pub")]
pub struct Span {
    /// The **start** of the span.
    ///
    /// # Example
    ///
    /// ```rust
    /// use leafc_utils::codemap::{
    ///     Span,
    ///     TextPosition,
    /// };
    ///
    /// let span = Span::new(0..10);
    ///
    /// assert_eq!(TextPosition::from(0), span.start());
    /// ```
    start: TextPosition,

    /// The **end** of the span.
    ///
    /// # Example
    ///
    /// ```rust
    /// use leafc_utils::codemap::{
    ///     Span,
    ///     TextPosition,
    /// };
    ///
    /// let span = Span::new(0..10);
    ///
    /// assert_eq!(TextPosition::from(10), span.end());
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
    /// use std::ops::Range;
    ///
    /// let span = Span::new(0..10);
    /// let range: Range<usize> = span.into();
    ///
    /// assert_eq!(range, 0..10);
    /// ```
    fn into(self) -> Range<usize> {
        self.start().into()..self.end().into()
    }
}

impl From<Range<usize>> for Span {
    /// Creates a new [`Span`] from the given [`Range<usize>`][`Range`].
    /// Internally, this span is created within the context of an **interner**
    /// and the same span is returned if it _already exists_.
    ///
    /// # Example
    ///
    /// ```rust
    /// use leafc_utils::Span;
    ///
    /// let span = Span::from(0..10);
    ///
    /// assert_eq!(span, Span::new(0..10));
    /// assert_eq!(span, Span::from(0..10));
    /// assert_eq!(span, Span::from(0..=9));
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
    /// use leafc_utils::codemap::{
    ///     Span,
    ///     TextPosition,
    /// };
    ///
    /// let span = Span::new(0..10);
    ///
    /// assert_eq!(TextPosition::from(0), span.start());
    /// assert_eq!(TextPosition::from(10), span.end());
    ///
    /// let span = Span::new(0..0);
    ///
    /// assert_eq!(TextPosition::from(0), span.start());
    /// assert_eq!(TextPosition::from(0), span.end());
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
    /// use leafc_utils::codemap::{
    ///     Span,
    ///     TextPosition,
    /// };
    ///
    /// let mut span = Span::new(0..10);
    ///
    /// assert_eq!(TextPosition::from(0), span.start());
    ///
    /// span.set_start(5);
    ///
    /// assert_eq!(TextPosition::from(5), span.start());
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
    /// use leafc_utils::codemap::{
    ///     Span,
    ///     TextPosition,
    /// };
    ///
    /// let mut span = Span::new(0..10);
    ///
    /// assert_eq!(TextPosition::from(10), span.end());
    ///
    /// span.set_end(15);
    ///
    /// assert_eq!(TextPosition::from(15), span.end());
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
    /// let span = Span::new(0..10);
    ///
    /// assert_eq!(10, span.len());
    ///
    /// let span = Span::new(0..0);
    ///
    /// assert_eq!(0, span.len());
    /// ```
    #[inline]
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
    /// let span = Span::new(0..10);
    ///
    /// assert!(!span.is_empty());
    ///
    /// let span = Span::new(0..0);
    ///
    /// assert!(span.is_empty());
    /// ```
    #[inline]
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
    /// let span = Span::new(0..10);
    ///
    /// assert!(span.contains(&Span::from(0..10)));
    /// assert!(span.contains(&Span::from(0..5)));
    ///
    /// assert!(!span.contains(&Span::from(0..15)));
    /// assert!(!span.contains(&Span::from(5..15)));
    /// ```
    #[inline]
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
    /// let span = Span::new(0..10);
    ///
    /// assert!(span.contains_index(5));
    /// assert!(!span.contains_index(15));
    /// ```
    #[inline]
    #[must_use]
    pub fn contains_index(&self, index: impl Into<TextPosition>) -> bool {
        let index = index.into();
        self.start <= index && self.end >= index
    }

    /// Returns the **range** of the span.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::{
    ///     Span,
    ///     TextPosition,
    /// };
    ///
    /// let span = Span::new(0..10);
    ///
    /// assert_eq!(TextPosition::from(0)..TextPosition::from(10), span.range());
    /// ```
    #[inline]
    #[must_use]
    pub fn range(&self) -> Range<TextPosition> {
        self.start()..self.end()
    }

    /// Merge the given `other` span into the span.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::{
    ///     Span,
    ///     TextPosition,
    /// };
    ///
    /// let mut span = Span::new(0..10);
    ///
    /// span.merge(&Span::new(5..15));
    ///
    /// assert_eq!(TextPosition::from(0)..TextPosition::from(15), span.range());
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

/// A **spanned** node (useful when working in the context of ASTs).
/// This is a node with associated span information.
/// This is useful for storing the location of a node with an
/// **arbitrary** type `T` within a source file or within the
/// context of a REPL.
///
/// # Example
///
/// ```
/// use leafc_utils::codemap::{
///     Span,
///     Spanned,
///     TextPosition,
/// };
///
/// // Create a spanned node, with a span of `0..12`. Here, we're
/// // just spanning a string, for simplicity, but you could imagine
/// // this being a node in an AST, a token in a token stream, or some
/// // other associated data structure (e.g. error messages / general diagnostics)
/// let mut spanned = Spanned::new(Span::from(0..12), "Hello, World!");
///
/// // Get the inner node.
/// assert_eq!("Hello, World!", spanned.node());
/// // Get the span.
/// assert_eq!(0..12, spanned.span_range());
///
/// // Set the node.
/// spanned.set_node("Hello, Universe!");
///
/// // The span is unchanged, but the hode is.
/// assert_eq!("Hello, Universe!", spanned.node());
/// assert_eq!(0..12, spanned.span_range());
///
/// // Set the span.
/// spanned.set_span(Span::from(5..15));
///
/// assert_eq!(TextPosition::from(5), spanned.span_start());
/// assert_eq!(TextPosition::from(15), spanned.span_end());
///
/// // The value is unchanged, but the span is.
/// assert_eq!(10, spanned.span_size());
///
/// // And so on ...
///
/// spanned.set_span_start(TextPosition::from(10));
/// spanned.set_span_end(TextPosition::from(21));
///
/// assert_eq!(11, spanned.span_size());
/// ```
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, CopyGetters, MutGetters, Dupe,
)]
#[getset(get = "pub", get_mut = "pub")]
pub struct Spanned<T> {
    /// The span of the node.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::{
    ///     Span,
    ///     Spanned,
    /// };
    ///
    /// let spanned = Spanned::new(Span::from(0..10), 5);
    ///
    /// assert_eq!(Span::from(0..10), spanned.span());
    /// ```
    pub span: Span,

    /// The _spanned_ node.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::{
    ///     Span,
    ///     Spanned,
    /// };
    ///
    /// let spanned = Spanned::new(Span::from(0..4), 2013);
    ///
    /// assert_eq!(2013, spanned.node());
    /// ```
    pub node: T,
}

#[test]
fn spanned_smoke_test() {
    let mut spanned = Spanned::new(Span::from(0..12), "Hello, World!");

    assert_eq!("Hello, World!".to_string(), (*spanned.node()).to_string());
    assert_eq!(Span::from(0..12), spanned.span());

    assert_eq!(spanned.span_range(), 0..12);

    spanned.set_span(Span::from(5..15));
    assert_eq!(spanned.span_start(), TextPosition::from(5));
    spanned.set_node("Hello, Universe!");
}

impl<T: Dupe> Spanned<T> {
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
    /// let spanned = Spanned::new(Span::from(0..10), 5);
    ///
    /// assert_eq!(Span::from(0..10), spanned.span());
    /// assert_eq!(5, spanned.node());
    /// ```
    #[must_use]
    pub const fn new(span: Span, value: T) -> Self {
        Self { span, node: value }
    }

    /// Returns the **node** contained within the span.
    /// This is the same as `self.node`.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::{
    ///     Span,
    ///     Spanned,
    /// };
    ///
    /// let spanned = Spanned::new(Span::from(0..10), 5);
    ///
    /// assert_eq!(5, spanned.node());
    /// ```
    #[inline]
    #[must_use]
    pub fn node(&self) -> T {
        self.node.dupe()
    }

    /// Returns the **span** of the node.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::{
    ///     Span,
    ///     Spanned,
    /// };
    ///
    /// let spanned = Spanned::new(Span::from(0..10), 5);
    ///
    /// assert_eq!(Span::from(0..10), spanned.span());
    /// ```
    #[inline]
    #[must_use]
    pub const fn span(&self) -> Span {
        self.span
    }

    /// Sets the **node** contained within the span.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::{
    ///     Span,
    ///     Spanned,
    /// };
    ///
    /// let mut spanned = Spanned::new(Span::from(0..10), 5);
    ///
    /// assert_eq!(5, spanned.node());
    ///
    /// spanned.set_node(10);
    ///
    /// assert_eq!(10, spanned.node());
    /// ```
    #[inline]
    pub fn set_node(&mut self, value: T) {
        self.node = value;
    }

    /// Sets the **span** of the node.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::{
    ///     Span,
    ///     Spanned,
    /// };
    ///
    /// let mut spanned = Spanned::new(Span::from(0..10), 5);
    ///
    /// assert_eq!(Span::from(0..10), spanned.span());
    ///
    /// spanned.set_span(Span::from(5..15));
    ///
    /// assert_eq!(Span::from(5..15), spanned.span());
    /// ```
    #[inline]
    pub fn set_span(&mut self, span: Span) {
        self.span = span;
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
    ///     TextPosition,
    /// };
    ///
    /// let spanned = Spanned::new(Span::from(0..10), 5);
    ///
    /// assert_eq!(TextPosition::from(0), spanned.span_start());
    /// ```
    #[inline]
    #[must_use]
    pub fn span_start(&self) -> TextPosition {
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
    ///     TextPosition,
    /// };
    ///
    /// let spanned = Spanned::new(Span::from(0..10), 5);
    ///
    /// assert_eq!(TextPosition::from(10), spanned.span_end());
    /// ```
    #[inline]
    #[must_use]
    pub fn span_end(&self) -> TextPosition {
        self.span.end()
    }

    /// Sets the **start** of the span.
    ///
    /// _This is the same as `self.span.set_start(start)`._
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::{
    ///     Span,
    ///     Spanned,
    ///     TextPosition,
    /// };
    ///
    /// let mut spanned = Spanned::new(Span::from(0..10), 5);
    ///
    /// spanned.set_span_start(TextPosition::from(5));
    /// assert_eq!(TextPosition::from(5), spanned.span_start());
    /// ```
    #[inline]
    pub fn set_span_start(&mut self, start: TextPosition) {
        self.span.set_start(start);
    }

    /// Sets the **end** of the span.
    ///
    /// _This is the same as `self.span.set_end(end)`._
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::{
    ///     Span,
    ///     Spanned,
    ///     TextPosition,
    /// };
    ///
    /// let mut spanned = Spanned::new(Span::from(0..10), 5);
    ///
    /// spanned.set_span_end(TextPosition::from(15));
    ///
    /// assert_eq!(TextPosition::from(15), spanned.span_end());
    /// ```
    #[inline]
    pub fn set_span_end(&mut self, end: impl Into<TextPosition>) {
        self.span.set_end(end);
    }

    /// Returns the **size** of the span.
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
    /// let spanned = Spanned::new(Span::new(0..10), 5);
    ///
    /// assert_eq!(10, spanned.span_size());
    /// ```
    #[inline]
    #[must_use]
    pub fn span_size(&self) -> usize {
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
    /// let spanned = Spanned::new(Span::new(0..10), 5);
    ///
    /// assert!(!spanned.span_is_empty());
    /// ```
    #[inline]
    #[must_use]
    pub fn span_is_empty(&self) -> bool {
        self.span.is_empty()
    }

    /// Returns the **range of the span**.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::{
    ///     Span,
    ///     Spanned,
    /// };
    ///
    /// let spanned = Spanned::new(Span::new(0..10), 5);
    ///
    /// assert_eq!(0..10, spanned.span_range());
    /// ```
    #[inline]
    #[must_use]
    pub fn span_range(&self) -> Range<usize> {
        let start = self.span.start().into();
        let end = self.span.end().into();

        start..end
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
    /// let spanned = Spanned::new(Span::new(0..10), 5);
    ///
    /// let spanned = spanned.map(|value| value + 1);
    ///
    /// assert_eq!(6, spanned.node());
    /// ```
    pub fn into_map<U: Dupe>(self, f: impl FnOnce(T) -> U) -> Spanned<U> {
        Spanned::new(self.span, f(self.node))
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
    /// let spanned = Spanned::new(Span::new(0..10), 5);
    ///
    /// let spanned = spanned.map(|value| value + 1);
    ///
    /// assert_eq!(6, spanned.node());
    /// ```
    pub fn map<U: Dupe>(&self, f: impl FnOnce(&T) -> U) -> Spanned<U> {
        Spanned::new(self.span, f(&self.node))
    }
}

impl<T> Deref for Spanned<T> {
    type Target = T;

    /// Returns a reference to the **contained value**.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::codemap::{
    ///    Span,
    ///   Spanned,
    /// };
    ///
    /// let spanned = Spanned::new(Span::new(0..10), 5);
    /// let value = *spanned;
    ///
    /// assert_eq!(5, value);
    fn deref(&self) -> &T {
        &self.node
    }
}

impl<T> DerefMut for Spanned<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.node
    }
}
