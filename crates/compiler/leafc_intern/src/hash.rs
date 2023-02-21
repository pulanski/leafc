use std::hash::BuildHasherDefault;

use fxhash::FxBuildHasher;

/// The **cryptographically insecure** hasher used by throughout the compiler.
/// This is a **fast** hasher that is used for data structures that do not need
/// to be cryptographically secure (e.g. **interned strings**).
///
/// // TODO: ability to change/configure the insecure hasher
pub type InsecureHasher = FxBuildHasher;

/// The **cryptographically secure** hasher used by throughout the compiler.
/// This is a hasher which is **slower** than the `InsecureHasher`, but has
/// the benefit of being **cryptographically secure**.
pub type SecureHasher = BuildHasherDefault<Blake3Hasher>;

/// Type alias for the **Blake3** hasher.
pub type Blake3Hasher = blake3::Hasher;
