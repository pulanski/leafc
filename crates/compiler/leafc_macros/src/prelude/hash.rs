#[doc(hidden)]
mod cfg {
    pub use hasher::RandomState;

    #[doc(hidden)]
    mod hasher {
        pub use cfg_if::cfg_if;

        // Configure the hasher used by the `HashMap` and `HashSet` types.
        cfg_if! {
            if #[cfg(feature = "ahasher")] {
                pub use ahash::RandomState;
            } else {
                pub use std::collections::hash_map::RandomState;
            }
        }
    }
}

pub use cfg::RandomState;
