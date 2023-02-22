use std::collections::VecDeque;

use cfg_if::cfg_if;
use derive_more::Display;
use derive_new::new;
use getset::{
    CopyGetters,
    Getters,
    MutGetters,
    Setters,
};
use rusty_ulid::Ulid;
use smartstring::alias::String;
use smol_str::SmolStr;

use leafc_intern::string::StringId;
use leafc_macros::LEAFC_FEATURE_USE_DECLS;

use super::text::TextPosition;

LEAFC_FEATURE_USE_DECLS!();

// TODO: refactor to encapsulated abstraction for multi-threaded and
// single-threaded use cases
cfg_if! {
    if #[cfg(feature = "multi-threaded")] {
        // use crossbeam::atomic::AtomicCell; // TODO: see if we can integrate this at all
        // use crossbeam::atomic::Ordering;
        // use crossbeam::queue::SegQueue;
        // use crossbeam::queue::SegQueueIter;


        use std::sync::Arc;

        use parking_lot::Mutex;
    }
}

// TODO: source text string in general should be interned within data structures
// that use it across the compiler.

/// A **unique identifier** for a **file**.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, new, Display)]
#[allow(clippy::module_name_repetitions)]
#[new]
#[display(fmt = "{_0}")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "allocative", derive(Allocative))]
#[repr(transparent)]
pub struct FileId(#[new(value = "Ulid::generate()")] Ulid);
// TODO: refactor this to be an interned data structure

/// Internal data structure for a **file**. This is **not** the public API for
/// a file and should **not** be used directly. Use the [`File`] API instead.
///
/// This is the core data structure used in both the **multi-threaded** and
/// **single-threaded** contexts.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Getters, CopyGetters, MutGetters, Setters)]
// #[derive(new)]
#[getset(get_mut = "pub", set = "pub")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[allow(clippy::module_name_repetitions)]
pub struct FileData {
    /// The **unique identifier** for the file.
    #[getset(get_copy = "pub")]
    file_id:     FileId,
    /// The **name** of the file.
    /// This is the name of the file used in **error messages**.
    /// and **diagnostics**. This is **not** the **absolute path** to the file.
    #[getset(get = "pub")]
    name:        SmolStr,
    /// The **absolute path** to the file on the filesystem.
    #[getset(get = "pub")]
    abs_path:    SmolStr,
    /// The **source text** contained within the file.
    #[getset(get = "pub")]
    source_text: StringId,
    /// The **start indices** of each line in the file.
    /// This is used to **efficiently** convert between **spans** and
    /// **line/column positions**.
    #[getset(get_mut = "pub")]
    line_starts: VecDeque<TextPosition>,
}

#[allow(missing_docs)] // allowed for internal data structures
impl FileData {
    pub fn new(
        // file_id: impl Into<FileId>,
        abs_path: impl Into<SmolStr> + Clone,
        source_text: impl Into<String>,
    ) -> Self {
        // with ctxt: Ctxt {
        let mut line_starts = VecDeque::new();
        let source_text: String = source_text.into();
        line_starts.push_back(TextPosition::new(0));
        line_starts.extend(source_text.match_indices('\n').map(|(i, _)| TextPosition::new(i + 1)));

        let path = abs_path.clone().into();
        let file_id = FileId::new();
        let filename = path.split('/').last().unwrap_or_else(|| path.as_str());

        // let source_text = StringId::intern(source_text);

        Self {
            file_id,
            abs_path: abs_path.into(),
            source_text: StringId::new(0),
            // source_text,
            line_starts,
            name: filename.into(),
        }
    }

    #[must_use]
    pub fn line_start(&self, line_index: usize) -> Option<TextPosition> {
        use std::cmp::Ordering;

        // TODO: maybe in the future track EOF

        match line_index.cmp(&self.line_starts.len()) {
            Ordering::Less => Some(self.line_starts[line_index]),
            Ordering::Equal | Ordering::Greater => None,
        }
    }
}

/// A **thread-safe** [`File`]. This is used to **share** a **file** between
/// **threads**. For example, this is used to **share** a **file** between
/// different components of the compiler (e.g. the **lexer**, **parser**,
/// **typechecker**, etc.) for reducing the **memory footprint** of
/// the compiler.
///
/// **NOTE**: This is used when the `multi-threaded` feature is enabled.
///
/// A [`File`] contains the **source text** of a **file** and is used to
#[cfg(feature = "multi-threaded")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct File(Arc<FileData>);

/// A **non-thread-safe** [`File`].
///
/// **NOTE**: **This is the default** and is used when the `multi-threaded`
/// feature is **not** enabled.
///
/// A [`File`] contains the **source text** of a **file** and is used to
/// **efficiently** convert between **spans** and **line/column positions**.
/// This is the **primary** data structure used in the compiler for
/// **diagnostics** and **error messages** emitted from the frontend.
#[cfg(not(feature = "multi-threaded"))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct File(FileData);

impl From<FileData> for File {
    fn from(file_data: FileData) -> Self {
        #[cfg(feature = "multi-threaded")]
        return Self(Arc::new(file_data));

        #[cfg(not(feature = "multi-threaded"))]
        return Self(file_data);
    }
}

impl From<(&str, &str)> for File {
    fn from((abs_path, source_text): (&str, &str)) -> Self {
        // TODO: refactor to intern the entire file.
        // interner is a singleton exposed by the interner crate
        // and used within the context of the driver (i.e. the
        // driver sets up the interner and passes it to the
        // codemap)
        Self::new(abs_path, source_text)
    }
}

impl File {
    /// Creates a new [`File`] from the given [`FileData`].
    ///
    /// # Arguments
    ///
    /// // TODO: use ulid under the hood to auto-generate `file_id` that is
    /// unique // across all files
    /// * `file_id` - The **unique identifier** for the file.
    ///
    /// * `abs_path` - The **absolute path** to the file on the filesystem.
    ///
    /// * `source_text` - The **source text** contained within the file.
    ///
    /// # Examples
    ///
    /// ```
    /// use leafc_utils::codemap::{
    ///     File,
    ///     FileData,
    /// };
    ///
    /// let file = File::new("foo", "bar");
    ///
    /// assert_eq!(file.abs_path(), "foo");
    /// assert_eq!(file.source_text(), "bar");
    /// ```
    #[must_use]
    pub fn new(abs_path: impl Into<SmolStr> + Clone, source_text: impl Into<String>) -> Self {
        #[cfg(feature = "multi-threaded")]
        return Self(Arc::new(FileData::new(abs_path, source_text)));

        #[cfg(not(feature = "multi-threaded"))]
        Self(FileData::new(abs_path, source_text))
    }

    /// Returns the **unique identifier** for the file.
    #[must_use]
    pub fn file_id(&self) -> FileId {
        self.0.file_id()
    }

    /// Returns the **absolute path** to the file on the filesystem.
    #[must_use]
    pub fn abs_path(&self) -> &SmolStr {
        self.0.abs_path()
    }

    /// Returns the **source text** contained within the file.
    #[must_use]
    pub const fn source_text(&self) -> &str {
        "" // TODO: implement
           // self.0.source_text()
           // self.0.source_text()
           // interner.get(self.0.source_text())
    }

    /// Returns the **starting byte index** of the given `line_index` in this
    /// [`File`].
    #[must_use]
    pub fn line_start(&self, line_index: usize) -> Option<TextPosition> {
        self.0.line_start(line_index)
    }
}

/// A **collection** of [`File`]s. This is used to **store** the **contents**
/// of all files that are **loaded** into the compiler at once for a given
/// program.
///
/// # Examples
///
/// ```rust,ignore
/// use leafc_utils::location::FileSet;
///
/// let mut file_set = FileSet::new();
/// let file_id = file_set.add_file("foo", "bar");
/// let file = file_set.get_file(file_id);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Getters, CopyGetters, MutGetters, Setters)]
#[allow(clippy::module_name_repetitions)]
pub struct FileSet {
    /// The **files** that are **contained** within this [`FileSet`].
    #[getset(get = "pub", get_mut = "pub")]
    files: VecDeque<File>,

    /// A **cursor** that is used to **track** the **current file** that is
    /// being **processed**. Defaults to `None`, meaning that no file is
    /// in the **file set**.
    #[getset(get_copy = "pub", set = "pub")]
    cursor: Option<FileId>,

    /// The **absolute path** to the **current working directory**.
    #[getset(get = "pub", set = "pub")]
    cwd: SmolStr,
}

impl Default for FileSet {
    /// Creates a new [`FileSet`] with the **current working directory** set
    /// to the **current working directory** of the process.
    ///
    /// # Examples
    ///
    /// ```
    /// use leafc_utils::location::FileSet;
    ///
    /// let file_set = FileSet::default();
    ///
    /// assert_eq!(file_set.cwd(), std::env::current_dir().unwrap().to_str().unwrap());
    /// assert_eq!(file_set.files().len(), 0);
    /// assert_eq!(file_set.next_file_id(), 0);
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl FileSet {
    /// Creates a new [`FileSet`] with the **current working directory** set
    /// to the **current working directory** of the process and **no files**
    /// loaded.
    ///
    /// # Examples
    ///
    /// ```
    /// // TODO: add example
    /// ```
    ///
    /// # Panics
    ///
    /// This function will panic if the **current working directory** of the
    /// process cannot be determined or **does not exist**.
    #[must_use]
    pub fn new() -> Self {
        Self {
            files:  VecDeque::new(),
            cursor: None,
            cwd:    std::env::current_dir().unwrap().to_string_lossy().into_owned().into(),
        }
    }

    /// Adds a new [`File`] with the given **absolute path** and **source
    /// text** to this [`FileSet`], returning the [`FileId`] of the newly
    /// added file.
    pub fn add_file(
        &mut self,
        abs_path: impl Into<SmolStr> + Clone,
        source_text: impl Into<String>,
    ) -> FileId {
        File::new(abs_path, source_text).file_id()
    }

    /// Returns the [`File`] with the given `file_id` from this [`FileSet`].
    #[inline]
    #[must_use]
    pub const fn get_file(&self, file_id: FileId) -> Option<&File> {
        // self.files.get(file_id.0).map(File::new)
        None // TODO: implement
             // self.files.get(file_id.0.as_size()).map(File::new)
    }
}

#[cfg(test)]
mod file_test_suite {
    use super::*;
    use pretty_assertions_sorted::assert_eq;

    #[test]
    fn test_file_line_start() {
        let file_one = File::new("foo", "bar");
        // let file_one = File::from("foo", "bar");

        assert_eq!(file_one.line_start(0), Some(TextPosition::from(0)));
        assert_eq!(file_one.line_start(1), None);

        // File::from() will either create a new file or return an existing
        // interned file with the same absolute path and source text.
        let file_two = File::new(
            "foo",
            "abcdef
789
123456
8920",
        );

        assert_eq!(file_two.line_start(0), Some(TextPosition::from(0)));
        assert_eq!(file_two.line_start(1), Some(TextPosition::from(7)));
        assert_eq!(file_two.line_start(2), Some(TextPosition::from(11)));
        assert_eq!(file_two.line_start(3), Some(TextPosition::from(18)));
        assert_eq!(file_two.line_start(4), None);

        let file = File::new(
            "foo",
            "bar



        baz",
        );

        assert_eq!(file.line_start(0), Some(TextPosition::from(0)));
        assert_eq!(file.line_start(1), Some(TextPosition::from(4)));
        assert_eq!(file.line_start(2), Some(TextPosition::from(5)));
        assert_eq!(file.line_start(3), Some(TextPosition::from(6)));
        assert_eq!(file.line_start(4), Some(TextPosition::from(7)));
        assert_eq!(file.line_start(5), None);
    }

    #[test]
    fn test_file_set() {
        let file_set = FileSet::new();

        assert_eq!(file_set.files(), &[]);
        assert_eq!(file_set.cursor(), None);
        assert_eq!(file_set.cwd(), std::env::current_dir().unwrap().to_str().unwrap());

        let mut file_set = FileSet::new();

        assert_eq!(file_set.files(), &[]);
        assert_eq!(file_set.cursor(), None);
        assert_eq!(file_set.cwd(), std::env::current_dir().unwrap().to_str().unwrap());

        // add a file...

        let file_id = file_set.add_file("foo", "bar");

        // assert_eq!(file_set.files(), &[File::new("foo", "bar")]);
        assert_eq!(file_set.files(), &[File::from(("foo", "bar"))]);
        assert_eq!(file_set.cursor(), None);
        assert_eq!(file_set.get_file(file_id), Some(&File::new("foo", "bar")));
        // assert_eq!(file_set.get_file(file_id), Some(&File::from("foo", "bar")));

        // add another file ...
        let file_id = file_set.add_file("baz", "qux");

        assert_eq!(file_set.files(), &[File::new("foo", "bar"), File::new("baz", "qux")]);
    }
}

/// A combination of a **line** and **column** in a source file.
///
/// This is used to provide **context** for error messages and
/// to **highlight** the source code.
///
/// # Example
///
/// ```ignore
/// use leafc_lexer::TokenStream;
///
/// let input = "x := 5;";
/// let tokens = TokenStream::new(input);
///
/// // TODO: add example
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, CopyGetters, MutGetters, Setters, new)]
#[allow(clippy::module_name_repetitions)]
pub struct LineColumn {
    /// The **line** of the position.
    line:   usize,
    /// The **column** of the position.
    column: usize,
}

impl From<(usize, usize)> for LineColumn {
    /// Converts a **tuple** of the **line** and **offset** to a `LineColumn`.
    /// The **tuple** is of the form `(line, offset)`.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::LineColumn;
    ///
    /// let (line, offset) = (0, 0);
    /// let file_position = LineColumn::from((line, offset));
    /// ```
    fn from((line, offset): (usize, usize)) -> Self {
        Self { line, column: offset }
    }
}

impl From<LineColumn> for (usize, usize) {
    /// Converts a `LineColumn` to a **tuple** of the **line** and **offset**.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_utils::LineColumn;
    ///
    /// let file_position = LineColumn::new(0, 0);
    /// let (line, offset) = file_position.into();
    /// ```
    fn from(position: LineColumn) -> Self {
        (position.line, position.column)
    }
}
