use crate::hash::{
    InsecureHasher,
    SecureHasher,
};
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "multi-threaded")] {
        use dashmap::DashMap;

        /// A **thread-safe hash map** that uses the compiler's **insecure** hasher.
        ///
        /// This is only available when the `multi-threaded` feature is **enabled**.
        ///
        /// **NOTE**: This is **not** secure, and should **not** be used for anything
        /// other than **internal** compiler data structures.
        pub type HashMap<K, V> = DashMap<K, V, InsecureHasher>;

        /// A **thread-safe hash set** that uses the compiler's **insecure** hasher.
        ///
        /// This is only available when the `multi-threaded` feature is **enabled**.
        ///
        /// **NOTE**: This is **not** secure, and should **not** be used for anything
        /// other than **internal** compiler data structures.
        pub type HashSet<K> = DashSet<K, InsecureHasher>;

        /// A **thread-safe hash map** that uses the compiler's **secure** hasher.
        ///
        /// This is only available when the `multi-threaded` feature is **enabled**.
        ///
        /// **NOTE**: This is **secure**, and should be used for anything that needs to
        /// be **secure** (e.g. **user** data in a build system context).
        pub type SecureHashMap<K, V> = DashMap<K, V, SecureHasher>;

        /// A **thread-safe hash set** that uses the compiler's **secure** hasher.
        ///
        /// This is only available when the `multi-threaded` feature is **enabled**.
        ///
        /// **NOTE**: This is **secure**, and should be used for anything that needs to
        /// be **secure** (e.g. **user** data in a build system context).
        pub type SecureHashSet<K> = DashSet<K, SecureHasher>;
    } else {
        /// A **hash map** that uses the compiler's **insecure** hasher.
        ///
        /// **NOTE**: This is **not** secure, and should **not** be used for anything
        /// other than **internal** compiler data structures. Additionally, this is **not**
        /// thread-safe, and should **not** be used in a multi-threaded context. If you
        /// need a thread-safe hash map, enable the `multi-threaded` feature.
        pub type HashMap<K, V> = hashbrown::HashMap<K, V, InsecureHasher>;

        /// A **hash set** that uses the compiler's **insecure** hasher.
        ///
        /// **NOTE**: This is **not** secure, and should **not** be used for anything
        /// other than **internal** compiler data structures. Additionally, this is **not**
        /// thread-safe, and should **not** be used in a multi-threaded context. If you
        /// need a thread-safe hash set, enable the `multi-threaded` feature.
        pub type HashSet<K> = hashbrown::HashSet<K, InsecureHasher>;

        /// A **hash map** that uses the compiler's **secure** hasher.
        ///
        /// **NOTE**: This is **secure**, and should be used for anything that needs to
        /// be **secure** (e.g. **user** data in a build system context). Additionally,
        /// this is **not** thread-safe, and should **not** be used in a multi-threaded
        /// context. If you need a thread-safe hash map, enable the `multi-threaded` feature.
        pub type SecureHashMap<K, V> = hashbrown::HashMap<K, V, SecureHasher>;

        /// A **hash set** that uses the compiler's **secure** hasher.
        ///
        /// **NOTE**: This is **secure**, and should be used for anything that needs to
        /// be **secure** (e.g. **user** data in a build system context). Additionally,
        /// this is **not** thread-safe, and should **not** be used in a multi-threaded
        /// context. If you need a thread-safe hash set, enable the `multi-threaded` feature.
        pub type SecureHashSet<K> = hashbrown::HashSet<K, SecureHasher>;
    }
}
