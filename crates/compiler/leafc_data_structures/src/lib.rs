/// Defines a collection of utilities for working with various types of
/// **collections** (e.g. [`Vec`], [`HashMap`][crate::collections::HashMap],
/// etc.) via more ergonomic APIs. These utilities are used as a **facade**
/// for the same API (e.g. [`HashMap`][crate::collections::HashMap]) in both
/// single-threaded and multi-threaded contexts.
pub mod collections;

/// Defines a collection of utilities for working with **hashes** (e.g.
/// [`Hash`], [`SecureHasher`][crate::hash::SecureHasher],
/// [`InsecureHasher`][crate::hash::InsecureHasher], etc).
pub mod hash;
