#![allow(clippy::module_name_repetitions)]
use std::{
    collections::BTreeMap,
    num::NonZeroUsize,
};

use crate::{
    LineColumn,
    Location,
};

/// A **symbol** is a **unique identifier** for a **name** in the source code.
/// This is used to **intern** and **lookup** names in the source code.
///
/// # Examples
///
/// ```rust,ignore
/// // TODO: add example
/// use leafc_utils::symbol_table::{Symbol, SymbolId, SymbolKind};
/// use leafc_utils::Location;
///
/// let symbol = Symbol::new(
///    SymbolId::new(42),
///   SymbolKind::Function,
///  None,
/// Vec::new(),
/// Location::new(0, 0),
/// LineColumn::new(0, 0),
/// );
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    id:       SymbolId,
    /// The **kind** of the symbol (e.g. `SymbolKind::Function`).
    kind:     SymbolKind,
    /// The **parent** of the symbol (e.g. the `SymbolId` of the `struct` that
    /// contains the `fn`).
    // parent:   Option<SymbolId>,
    /// The **children** of the symbol (e.g. the `SymbolId`s of the `fn`s that
    /// are contained within the `struct`).
    // children: Vec<SymbolId>,
    /// The **location** of the symbol (e.g. the `Location` of the `fn`).
    location: Location,
    /// The **line and column** of the symbol (e.g. the `LineColumn` of the
    /// `fn`).
    line_col: LineColumn,
}

/// A **symbol table** is a **collection** of **symbols**. This is used to
/// **intern** and **lookup** symbols in the source code.
struct SymbolTable {
    pub symbols: BTreeMap<SymbolId, Symbol>,
}

/// A **unique identifier** for a **symbol**. This is used for **interning** and
/// **fast lookup** of symbols in the [`SymbolTable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SymbolId(NonZeroUsize);

impl SymbolId {
    /// Creates a new `SymbolId` from the given `usize`.
    ///
    /// # Arguments
    ///
    /// * `id` - The `usize` to create the `SymbolId` from. This **must** be
    /// greater than `0` and less than [`MAX_SYMBOL_ID`].
    ///
    /// # Panics
    ///
    /// Panics if `id` is `0` or greater than or equal to `usize::MAX`.
    ///
    /// # Examples
    #[inline]
    #[must_use]
    pub fn new(id: usize) -> Self {
        debug_assert!(id > 0 && id < usize::MAX);
        Self(NonZeroUsize::new(id).unwrap())
    }

    /// Returns the `usize` representation of the `FileId`.
    /// This is the **inverse** of [`SymbolId::new`] and is **only** used for
    /// **testing purposes**.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leafc_utils::symbol_table::SymbolId;
    /// use std::num::NonZeroUsize;
    ///
    /// let id = SymbolId::new(42);
    /// assert_eq!(id.as_usize(), 42);
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_usize(self) -> usize {
        self.0.get()
    }
}

/// The **maximum** `SymbolId` that can be created. This is used to
/// **determine** the **size** of the [`SymbolTable`]. This is set to
/// `usize::MAX - 1` to ensure that `0` can be used as a **sentinel value**.
pub static MAX_SYMBOL_ID: SymbolId = SymbolId(unsafe { NonZeroUsize::new_unchecked(usize::MAX) });

impl From<usize> for SymbolId {
    fn from(id: usize) -> Self {
        Self::new(id)
    }
}

/// The **kind** of a symbol (e.g. `SymbolKind::Function`). This is used to
/// **determine** the **type** of the symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SymbolKind {
    /// An attribute like `#[tokio::main]`.
    Attribute,
    /// A builtin attribute like `#[derive(Debug)]`.
    BuiltinAttr,
    /// A const like `const FOO: u32 = 42`.
    Const,
    /// An enum like `enum Foo { Bar }`.
    Enum,
    /// A field like `foo: u32`.
    Field,
    /// A function like `fn foo() {}`.
    Function,
    /// An impl like `impl Foo {}`.
    Impl,
    /// A label like `'foo: loop {}`.
    Label,
    /// A liftetime like `'foo`.
    LifetimeParam,
    /// A local variable like `let foo = 42`.
    Local,
    /// A macro like `macro_rules! foo {}`.
    Macro,
    /// A module like `mod foo {}`.
    Module,
    // /// A self parameter like `impl Foo for Bar {}`.
    // SelfParam,
    // /// A self type like `impl Foo for Bar {}`.
    // SelfType,
    /// A static like `static FOO: u32 = 42`.
    Static,
    /// A struct like `struct Foo {}`.
    Struct,
    /// A trait like `trait Foo {}`.
    Trait,
    /// A type alias like `type Foo = Bar`.
    TypeAlias,
    /// A type parameter like `struct Foo<T> {}`.
    TypeParam,
    /// A union like `union Foo {}`.
    Union,
    /// A value parameter like `fn foo<T>(t: T) {}`.
    ValueParam,
    /// A variant like `Foo::Bar`.
    Variant,
}
