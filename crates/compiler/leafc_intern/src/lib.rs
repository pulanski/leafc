#![feature(proc_macro_hygiene)]
#![feature(type_alias_impl_trait)]

#[cfg_attr(doc, aquamarine::aquamarine)]
/// Provides support for **interning strings**. This allows for strings to be
/// stored in a **single location**, and allows for **fast comparisons** between
/// strings.
///
/// ```mermaid
/// flowchart TD
///    A[Hello World] -->|Span: 0..10|D
///
///    B[Hello World] -->|Span: 22..32| D{StringInterner}
///    C[Hello world] -->|Span: 76..86| D{StringInterner}
///
///    D --> E[id: 0]
/// ```
///
/// # Examples
///
/// ```rust
/// use leafc_intern::string::StringInterner;
///
/// // Create a new string interner.
/// let interner = StringInterner::new();
///
/// // Intern a string.
/// let id = interner.intern("Hello, world!");
///
/// // Lookup the string from the interner.
/// assert_eq!(interner.lookup(id), "Hello, world!");
///
/// // Interning the same string will return the same id.
/// let id2 = interner.intern("Hello, world!");
///
/// // The ids are equal.
/// assert_eq!(id, id2);
///
/// // Interning a different string will return a different id.
/// let id3 = interner.intern("Hola, mundo!");
///
/// // The ids are not equal.
/// assert_ne!(id, id3);
/// ```
pub mod string;

/// Defines a collection of utilities for working with **hashes** (e.g.
/// [`Hash`], [`Hasher`][crate::hash::LeafcHasher], etc.).
pub mod hash;

// TODO: add generic support for interned values (e.g. `Interned<T>`)
