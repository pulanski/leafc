use leafc_intern::hash::InsecureHasher;

/// A **hash map** that uses the compiler's default hasher.
pub type HashMap<K, V> = hashbrown::HashMap<K, V, InsecureHasher>;

/// A **hash set** that uses the compiler's default hasher.
pub type HashSet<K> = hashbrown::HashSet<K, InsecureHasher>;
