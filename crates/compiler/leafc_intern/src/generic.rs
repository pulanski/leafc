#[cfg(feature = "multi-threaded")]
use dashmap::{
    DashMap,
    SharedValue,
};

#[cfg(feature = "multi-threaded")]
use std::sync::Arc;

#[cfg(feature = "multi-threaded")]
type InternMap<T> = DashMap<Arc<T>, (), BuildHasherDefault<FxHasher>>;
