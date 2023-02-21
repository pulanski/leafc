use cfg_if::cfg_if;
use leafc_data_structures::collections::HashMap;

// TODO: continue work on generic interning...
// supports generic interning via a unified API for both single-threaded and
// multi-threaded contexts.

cfg_if! {
    if #[cfg(feature = "multi-threaded")] {
        use std::sync::RwLock;
        use std::sync::RwLockWriteGuard;
        use std::sync::Arc;

        type InternMap<T> = RwLock<HashMap<Arc<T>, ()>>;
        type Guard<T> = RwLockWriteGuard<'static, HashMap<Arc<T>, ()>>;
    } else {
        type InternMap<T> = HashMap<Arc<T>, ()>;
        type Guard<T> = &'static mut HashMap<Arc<T>, ()>;
    }
}
