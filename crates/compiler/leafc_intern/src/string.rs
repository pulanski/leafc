#[cfg(feature = "multi-threaded")]
use lasso::ThreadedRodeo;
#[cfg(feature = "multi-threaded")]
use std::sync::Arc;

#[cfg(feature = "serde")]
use serde::{
    Deserialize,
    Serialize,
};

use leafc_data_structures::hash::InsecureHasher;

#[cfg(not(feature = "multi-threaded"))]
use std::{
    cell::RefCell,
    rc::Rc,
};

use std::{
    fmt::{
        Debug,
        Display,
    },
    mem,
};

use lasso::{
    Capacity,
    Key,
    Rodeo,
    Spur,
};

/// The **default** capacity of the string interner (i.e. the number of strings
/// it can hold).
const DEFAULT_CAPACITY: usize = 1000;

/// **Behavior** for being a string reference. That is, a type that can be
/// converted into a `&str`, and can be **displayed** and **debugged**.
pub trait StringRefImpl: AsRef<str> + Display + Debug {}
impl StringRefImpl for &str {}

/// A **unique identifier** for a string that has been interned. This allows
/// multiple strings to be stored in a single location, and allows for fast
/// comparisons between strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct StringId(Spur);

impl StringId {
    /// Creates a new `StringId` from a raw `usize`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// // TODO: add example
    /// use leafc_intern::string::StringId;
    ///
    /// let id = StringId::new(0);
    ///
    /// assert_eq!(id.get(), Spur::try_from_usize(0).unwrap());
    /// ```
    pub fn new(string: usize) -> Self {
        Self::from(Spur::try_from_usize(string).unwrap())
    }

    /// Returns the `Spur` associated with this `StringId`.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// use leafc_intern::string::StringId;
    ///
    /// let id = StringId::from_raw(0);
    ///
    /// assert_eq!(id.get(), Spur::from_usize(0));
    /// ```
    pub fn get(&self) -> Spur {
        self.0
    }

    /// Converts a `Spur` into a `StringId`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use lasso::{
    ///     Key,
    ///     Spur,
    /// };
    /// use leafc_intern::string::StringId;
    ///
    /// let id = StringId::from(Spur::try_from_usize(0).unwrap());
    ///
    /// assert_eq!(id.get(), Spur::try_from_usize(0).unwrap());
    /// ```
    pub fn from(spur: Spur) -> Self {
        // let spur = Spur::try_from_usize(
        Self(spur)
    }

    /// Converts a `StringId` into a raw `u32`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leafc_intern::string::StringId;
    ///
    /// let id = StringId::from_raw(1);
    ///
    /// assert_eq!(id.as_u32(), 1);
    /// ```
    pub fn as_u32(&self) -> u32 {
        unsafe { mem::transmute(self.0) }
    }

    /// Converts a raw `u32` into a `StringId`.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it does not check whether the `u32` is
    /// valid.
    ///
    /// # Panics
    ///
    /// This function will panic if the `u32` is not valid.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leafc_intern::string::StringId;
    ///
    /// let id = StringId::from_raw(1);
    ///
    /// assert_eq!(id.to_raw(), 1);
    /// ```
    pub fn from_raw(raw: u32) -> Self {
        unsafe { Self(mem::transmute(raw)) }
    }
}

/// A **multi-threaded** string interner. This allows multiple strings to be
/// stored in a single location, and allows for fast comparisons between
/// strings. This has the added benefit of allowing for multiple threads to
/// share the same string interner.
#[cfg(feature = "multi-threaded")]
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct StringInterner(Arc<ThreadedRodeo<Spur, InsecureHasher>>);

/// A **single-threaded** string interner. This allows multiple strings to be
/// stored in a single location, and allows for fast comparisons between
/// strings.
#[cfg(not(feature = "multi-threaded"))]
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct StringInterner(Rc<RefCell<Rodeo<Spur, InsecureHasher>>>);

impl StringInterner {
    ///////////////////////////////////////////////////////////////////////////
    // Multi-threaded string interning
    ///////////////////////////////////////////////////////////////////////////

    #[cfg(feature = "multi-threaded")]
    pub fn new() -> Self {
        Self(Arc::new(ThreadedRodeo::with_capacity_and_hasher(
            Capacity::for_strings(DEFAULT_CAPACITY),
            InsecureHasher::default(),
        )))
    }

    #[cfg(feature = "multi-threaded")]
    pub fn intern(&self, string: String) -> StringId {
        StringId(self.0.get_or_intern(string))
    }

    #[cfg(feature = "multi-threaded")]
    pub fn resolve(&self, id: StringId) -> &str {
        self.0.resolve(&id.0)
    }

    ///////////////////////////////////////////////////////////////////////////
    // Single-threaded string interning
    ///////////////////////////////////////////////////////////////////////////

    #[cfg(not(feature = "multi-threaded"))]
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(Rodeo::with_capacity_and_hasher(
            Capacity::for_strings(DEFAULT_CAPACITY),
            InsecureHasher::default(),
        ))))
    }

    #[cfg(not(feature = "multi-threaded"))]
    pub fn intern(&self, string: impl StringRefImpl) -> StringId {
        StringId::from(self.0.borrow_mut().get_or_intern(string))
    }

    #[cfg(not(feature = "multi-threaded"))]
    pub fn lookup(&self, id: StringId) -> &str {
        let borrow = self.0.borrow();
        let string: &str = borrow.resolve(&id.get());

        unsafe { core::mem::transmute(string) }
    }

    /// Interns a **static** string. This is useful for strings that are
    /// guaranteed to live for the duration of the program.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leafc_intern::string::StringInterner;
    ///
    /// static HELLO: &str = "hello";
    ///
    /// let interner = StringInterner::new();
    ///
    /// // Works with `&'static str`...
    /// let id = interner.intern_static(HELLO);
    ///
    /// assert_eq!(interner.lookup(id), HELLO);
    /// ```
    #[cfg(not(feature = "multi-threaded"))]
    pub fn intern_static(&self, string: &'static str) -> StringId {
        StringId(self.0.borrow_mut().get_or_intern_static(string))
    }
}

impl Default for StringInterner {
    fn default() -> Self {
        Self::new()
    }
}
