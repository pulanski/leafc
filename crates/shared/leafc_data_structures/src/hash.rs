use fxhash::FxBuildHasher;
use std::hash::BuildHasherDefault;

/// A builder for default **BLAKE3** hashers.
pub type Blake3BuildHasher = BuildHasherDefault<blake3::Hasher>;

/// The **cryptographically insecure** hasher used by throughout the compiler.
/// This is a **fast** hasher that is used for data structures that do not need
/// to be cryptographically secure (e.g. **interned strings**).
///
/// // TODO: ability to change/configure the insecure hasher
pub type InsecureHasher = FxBuildHasher;

/// The **cryptographically secure** hasher used by throughout the compiler.
/// This is a hasher which is **slower** than the `InsecureHasher`, but has
/// the benefit of being **cryptographically secure**.
pub type SecureHasher = Blake3BuildHasher;
