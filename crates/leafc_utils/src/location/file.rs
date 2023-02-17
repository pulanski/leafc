use std::collections::VecDeque;

use derive_new::new;
use getset::{
    CopyGetters,
    Getters,
    MutGetters,
    Setters,
};
use smartstring::alias::String;
use smol_str::SmolStr;

/// A **unique identifier** for a **file**.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, new)]
#[new]
#[allow(clippy::module_name_repetitions)]
pub struct FileId(pub usize);

/// A **file** that is **loaded** into the compiler.
///
/// This struct is used to **store** the **contents** of a file and to
/// **identify** the file within the compiler.
///
/// # Examples
///
/// ```
/// use leafc_utils::location::File;
///
/// let file = File::new(0, "foo", "bar");
///
/// assert_eq!(file.file_id(), 0);
/// assert_eq!(file.abs_path(), "foo");
/// assert_eq!(file.source_text(), "bar");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Getters, CopyGetters, MutGetters, Setters)]
// #[derive(Debug, Clone, PartialEq, Eq, Hash, new)]
#[getset(get_mut = "pub", set = "pub")]
pub struct File {
    /// The **unique identifier** for the file.
    #[getset(get_copy = "pub")]
    file_id:     FileId,
    /// The **absolute path** to the file on the filesystem.
    #[getset(get = "pub")]
    abs_path:    SmolStr,
    /// The **source text** contained within the file.
    #[getset(get = "pub")]
    source_text: String,
    /// The **start indices** of each line in the file.
    /// This is used to **efficiently** convert between **spans** and
    /// **line/column positions**.
    #[getset(get_mut = "pub")]
    line_starts: VecDeque<usize>,
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
    files: Vec<File>,

    /// The **next available file identifier**.
    #[getset(get_copy = "pub", set = "pub")]
    next_file_id: usize,

    /// The **absolute path** to the **current working directory**.
    #[getset(get = "pub", set = "pub")]
    cwd: SmolStr,
}

impl PartialEq<usize> for FileId {
    /// Returns `true` if the given `usize` is equal to the `file_id` of this
    /// [`FileId`].
    fn eq(&self, other: &usize) -> bool {
        self.0 == *other
    }
}

impl File {
    /// Creates a new [`File`] with the given **absolute path** and **source
    /// text**.
    ///
    /// # Arguments
    ///
    /// * `file_id` - The **unique identifier** for the file.
    ///
    /// * `abs_path` - The **absolute path** to the file on the filesystem.
    ///
    /// * `source_text` - The **source text** contained within the file.
    ///
    /// # Examples
    ///
    /// ```
    /// use leafc_utils::location::File;
    ///
    /// let file = File::new(0, "foo", "bar");
    ///
    /// assert_eq!(file.file_id(), 0);
    /// assert_eq!(file.abs_path(), "foo");
    /// assert_eq!(file.source_text(), "bar");
    /// ```
    pub fn new(
        file_id: usize,
        abs_path: impl Into<SmolStr>,
        source_text: impl Into<String>,
    ) -> Self {
        let mut line_starts = VecDeque::new();
        let source_text = source_text.into();

        let mut byte_count = 0;

        for line in source_text.lines() {
            // TODO: move this to tracing
            // eprintln!("line: {:?} ({} bytes)", line, line.len());

            line_starts.push_back(byte_count);
            byte_count += line.len() + 1; // add 1 to account for the
                                          // newline character
        }

        Self { file_id: FileId(file_id), abs_path: abs_path.into(), source_text, line_starts }
    }

    /// Returns the **starting byte index** of the given `line_index` in this
    /// [`File`].
    #[must_use]
    pub fn line_start(&self, line_index: usize) -> Option<usize> {
        use std::cmp::Ordering;

        // TODO: maybe in the future track EOF

        match line_index.cmp(&self.line_starts.len()) {
            Ordering::Less => Some(self.line_starts[line_index]),
            Ordering::Equal | Ordering::Greater => None,
        }
    }
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
            files:        Vec::new(),
            next_file_id: 0,
            cwd:          std::env::current_dir().unwrap().to_string_lossy().into_owned().into(),
        }
    }

    /// Adds a new [`File`] with the given **absolute path** and **source
    /// text** to this [`FileSet`].
    pub fn add_file(
        &mut self,
        abs_path: impl Into<SmolStr>,
        source_text: impl Into<String>,
    ) -> FileId {
        let file_id = self.next_file_id;
        self.next_file_id += 1;

        let file = File::new(file_id, abs_path, source_text);
        self.files.push(file);

        FileId(file_id)
    }

    /// Returns the [`File`] with the given `file_id` from this [`FileSet`].
    #[inline]
    #[must_use]
    pub fn get_file(&self, file_id: FileId) -> &File {
        &self.files[file_id.0]
    }
}

#[cfg(test)]
mod file_test_suite {
    use super::*;
    use pretty_assertions_sorted::assert_eq;

    #[test]
    fn test_file_line_start() {
        let file_one = File::new(0, "foo", "bar");

        assert_eq!(file_one.line_start(0), Some(0));
        assert_eq!(file_one.line_start(1), None);

        let file_two = File::new(
            0,
            "foo",
            "abcdef
789
123456
8920",
        );

        assert_eq!(file_two.line_start(0), Some(0));
        assert_eq!(file_two.line_start(1), Some(7));
        assert_eq!(file_two.line_start(2), Some(11));
        assert_eq!(file_two.line_start(3), Some(18));
        assert_eq!(file_two.line_start(4), None);

        let file = File::new(
            0,
            "foo",
            "bar



        baz",
        );

        assert_eq!(file.line_start(0), Some(0));
        assert_eq!(file.line_start(1), Some(4));
        assert_eq!(file.line_start(2), Some(5));
        assert_eq!(file.line_start(3), Some(6));
        assert_eq!(file.line_start(4), Some(7));
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

        assert_eq!(file_set.files(), &[File::new(0, "foo", "bar")]);
        assert_eq!(file_set.next_file_id(), 1);
        assert_eq!(file_set.get_file(file_id), &File::new(0, "foo", "bar"));

        // add another file

        let file_id = file_set.add_file("baz", "qux");

        assert_eq!(file_set.files(), &[File::new(0, "foo", "bar"), File::new(1, "baz", "qux")]);
    }
}
