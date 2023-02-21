use std::{
    collections::VecDeque,
    ops::{
        Add,
        AddAssign,
        Sub,
        SubAssign,
    },
};

// TODO: source text string in general should be interned within data structures
// that use it across the compiler.

// #[cfg(feature = "multi-threaded")]
// use crossbeam::atomic::AtomicCell;
// #[cfg(feature = "multi-threaded")]
// use crossbeam::atomic::Ordering;
// #[cfg(feature = "multi-threaded")]
// use crossbeam::queue::SegQueue;
// #[cfg(feature = "multi-threaded")]
// use crossbeam::queue::SegQueueError;
// #[cfg(feature = "multi-threaded")]
// use crossbeam::queue::SegQueueIter;
// #[cfg(feature = "multi-threaded")]
// use crossbeam::queue::SegQueueIterMut;
// #[cfg(feature = "multi-threaded")]
// use crossbeam::queue::SegQueuePeek;
// #[cfg(feature = "multi-threaded")]
// use crossbeam::queue::SegQueuePeekMut;

#[cfg(feature = "allocative")]
use allocative::Allocative;
use leafc_intern::string::StringId;
#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};
#[cfg(feature = "multi-threaded")]
use std::sync::Arc;

use amplify::confinement::Collection;
use derive_more::Display;
use derive_new::new;
use getset::{
    CopyGetters,
    Getters,
    MutGetters,
    Setters,
};
use smartstring::alias::String;
use smol_str::SmolStr;

use super::text::TextPosition;

/// A **unique identifier** for a **file**.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, new, Display, Allocative)]
#[new]
#[allow(clippy::module_name_repetitions)]
#[display(fmt = "{_0}")]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct FileId(pub usize);

impl Add<usize> for FileId {
    type Output = Self;

    /// Adds the given `usize` to the `file_id` of this [`FileId`].
    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub<usize> for FileId {
    type Output = Self;

    /// Subtracts the given `usize` from the `file_id` of this [`FileId`].
    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl AddAssign<usize> for FileId {
    /// Adds the given `usize` to the `file_id` of this [`FileId`].
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}

impl SubAssign<usize> for FileId {
    /// Subtracts the given `usize` from the `file_id` of this [`FileId`].
    fn sub_assign(&mut self, rhs: usize) {
        self.0 -= rhs;
    }
}

impl From<usize> for FileId {
    /// Creates a new [`FileId`] from the given `usize`.
    fn from(file_id: usize) -> Self {
        Self(file_id)
    }
}

impl PartialEq<usize> for FileId {
    /// Returns `true` if the given `usize` is equal to the `file_id` of this
    /// [`FileId`].
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

/// A **file** that is **loaded** into the compiler.
///
/// This struct is used to **store** the **contents** of a file and to
/// **identify** the file within the compiler.
///
/// # Examples
///
/// ```
/// use leafc_utils::codemap::File;
///
/// let file = File::new(0, "foo", "bar");
///
/// assert_eq!(file.file_id(), 0);
/// assert_eq!(file.abs_path(), "foo");
/// assert_eq!(file.source_text(), "bar");
/// ```
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

impl FileData {
    /// Creates a new [`File`] with the given **absolute path** and **source
    /// text**.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The **unique identifier** for the file.
    ///
    /// * `abs_path` - The **absolute path** to the file on the filesystem.
    ///
    /// * `source_text` - An **interned string** containing the **source text**
    ///  contained within the file.
    ///
    /// # Examples
    ///
    /// ```
    /// use leafc_utils::codemap::File;
    ///
    /// let file = File::new(0, "foo", "bar");
    ///
    /// assert_eq!(file.file_id(), 0);
    /// assert_eq!(file.abs_path(), "foo");
    /// assert_eq!(file.source_text(), "bar");
    /// ```
    pub fn new(
        file_id: impl Into<FileId>,
        abs_path: impl Into<SmolStr> + Clone,
        source_text: impl Into<String>,
    ) -> Self {
        // with ctxt: Ctxt {
        let mut line_starts = VecDeque::new();
        let source_text: String = source_text.into();
        line_starts.push_back(TextPosition::new(0));
        line_starts.extend(source_text.match_indices('\n').map(|(i, _)| TextPosition::new(i + 1)));

        let path = abs_path.clone().into();
        let filename = path.split('/').last().unwrap_or_else(|| path.as_str());

        // let source_text = StringId::intern(source_text);

        Self {
            file_id: file_id.into(),
            abs_path: abs_path.into(),
            source_text: StringId::new(0),
            // source_text,
            line_starts,
            name: filename.into(),
        }
    }

    /// Returns the **starting byte index** of the given `line_index` in this
    /// [`File`].
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

/// A **thread-safe** reference to a [`FileData`]. This is used to **share**
/// a **file** between **threads**.
#[cfg(feature = "multi-threaded")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct File(Arc<FileData>);

/// A **reference** to an inner [`FileData`]. This is used to **share** the same
/// interface as [`File`] as it's used in a **multi-threaded** context but
/// without the **thread-safety**.
#[cfg(not(feature = "multi-threaded"))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct File(FileData);

// #[cfg(not(feature = "multi-threaded"))]
// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct File(FileData);

impl From<FileData> for File {
    fn from(file_data: FileData) -> Self {
        #[cfg(feature = "multi-threaded")]
        return Self(Arc::new(file_data));

        #[cfg(not(feature = "multi-threaded"))]
        return Self(file_data);
    }
}

impl File {
    /// Creates a new [`File`] from the given [`FileData`].
    ///
    /// # Arguments
    ///
    /// * `file_data` - The [`FileData`] to create the [`File`] from.
    ///
    /// # Examples
    ///
    /// ```
    /// use leafc_utils::codemap::{
    ///     File,
    ///     FileData,
    /// };
    ///
    /// let file = File::new(0, "foo", "bar");
    ///
    /// assert_eq!(file.file_id(), 0);
    /// assert_eq!(file.abs_path(), "foo");
    /// assert_eq!(file.source_text(), "bar");
    /// ```
    #[must_use]
    pub fn new(
        file_id: impl Into<FileId>,
        abs_path: impl Into<SmolStr> + Clone,
        source_text: impl Into<String>,
    ) -> Self {
        #[cfg(feature = "multi-threaded")]
        return Self(Arc::new(FileData::new(file_id, abs_path, source_text)));

        #[cfg(not(feature = "multi-threaded"))]
        Self(FileData::new(file_id, abs_path, source_text))
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
    files: VecDeque<FileData>,

    /// The **next available file identifier**.
    #[getset(get_copy = "pub", set = "pub")]
    next_file_id: FileId,

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
            files:        VecDeque::new(),
            next_file_id: FileId(0),
            cwd:          std::env::current_dir().unwrap().to_string_lossy().into_owned().into(),
        }
    }

    /// Adds a new [`File`] with the given **absolute path** and **source
    /// text** to this [`FileSet`].
    pub fn add_file(
        &mut self,
        abs_path: impl Into<SmolStr> + Clone,
        source_text: impl Into<String>,
    ) -> FileId {
        let file_id = self.next_file_id;
        self.next_file_id += 1;

        let file = FileData::new(file_id.0, abs_path, source_text);
        self.files.push(file);

        file_id
    }

    /// Returns the [`File`] with the given `file_id` from this [`FileSet`].
    #[inline]
    #[must_use]
    pub fn get_file(&self, file_id: FileId) -> &FileData {
        &self.files[file_id.0]
    }
}

#[cfg(test)]
mod file_test_suite {
    use super::*;
    use pretty_assertions_sorted::assert_eq;

    #[test]
    fn test_file_line_start() {
        let file_one = FileData::new(0, "foo", "bar");

        assert_eq!(file_one.line_start(0), Some(TextPosition::from(0)));
        assert_eq!(file_one.line_start(1), None);

        let file_two = FileData::new(
            0,
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

        let file = FileData::new(
            0,
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
        let file_set = FileSet::default();

        assert_eq!(file_set.files(), &[]);
        assert_eq!(file_set.next_file_id(), 0);
        assert_eq!(file_set.cwd(), std::env::current_dir().unwrap().to_str().unwrap());

        let mut file_set = FileSet::new();

        assert_eq!(file_set.files(), &[]);
        assert_eq!(file_set.next_file_id(), 0);
        assert_eq!(file_set.cwd(), std::env::current_dir().unwrap().to_str().unwrap());

        // add a file...

        let file_id = file_set.add_file("foo", "bar");

        assert_eq!(file_set.files(), &[FileData::new(0, "foo", "bar")]);
        assert_eq!(file_set.next_file_id(), 1);
        assert_eq!(file_set.get_file(file_id), &FileData::new(0, "foo", "bar"));

        // add another file

        let file_id = file_set.add_file("baz", "qux");

        assert_eq!(file_set.files(), &[
            FileData::new(0, "foo", "bar"),
            FileData::new(1, "baz", "qux")
        ]);
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
