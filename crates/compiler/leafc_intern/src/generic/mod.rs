use std::{
    cell::OnceCell,
    hash::Hash,
    sync::Arc,
};

use dashmap::RwLockWriteGuard;
use leafc_data_structures::collections::HashMap;
// use cfg_if::cfg_if;
// use leafc_data_structures::collections::HashMap;

// TODO: continue work on generic interning...
// supports generic interning via a unified API for both single-threaded and
// multi-threaded contexts.

// TODO: refactor to multithreaded vs singlethreaded distinction
// cfg_if! {
//     if #[cfg(feature = "multi-threaded")] {
//         use std::sync::RwLock;
//         use std::sync::RwLockWriteGuard;
//         use std::sync::Arc;

//         type InternMap<T> = RwLock<HashMap<Arc<T>, ()>>;
//         type Guard<T> = RwLockWriteGuard<'static, HashMap<Arc<T>, ()>>;
//     } else {
//         type InternMap<T> = HashMap<Arc<T>, ()>;
//         type Guard<T> = &'static mut HashMap<Arc<T>, ()>;
//     }
// }

pub trait Internable: Hash + Eq + 'static {
    fn storage() -> &'static InternStorage<Self>;
}

type InternMap<T> = HashMap<Arc<T>, ()>;
type Guard<T> = RwLockWriteGuard<'static, HashMap<Arc<T>, ()>>;

pub struct InternStorage<T: ?Sized> {
    map: OnceCell<InternMap<T>>,
}

pub struct Interned<T: Internable + ?Sized> {
    value: Arc<T>,
}

impl<T: Internable + ?Sized> Interned<T> {
    pub fn new(value: Arc<T>) -> Self {
        // match Interned::lookup(&value) {
        //     Some(interned) => interned,
        //     None => Interned::insert(value),
        // }
        // Self { value }

        todo!()
    }

    /// Lookup the **given value** of type `T` in the interner.
    /// If the value is already interned, return the interned value.
    /// Otherwise, insert the value into the interner and return it.
    /// This is a **convenience method** that combines `lookup` and `insert`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use leafc_intern::generic::Interned;
    ///
    /// // Create a new string interner.
    /// let interner = Interned<File>::new();
    ///
    /// // Intern a file.
    /// let id = interner.intern(File::from("foo.rs")); // file should just be the name, not the path for from,
    ///                                                 // internally though it stores the path and other
    ///                                                 // bookkeeping
    /// ```
    pub fn lookup(value: &T) -> Result<Self, Guard<T>> {
        todo!()
    }
}

// impl<T: Internable + ?Sized> Interned<T> {
//     fn lookup(obj: &T) -> Result<Self, Guard<T>> {
//         let storage = T::storage().get();
//         let shard_idx = storage.determine_map(obj);
//         let shard = &storage.shards()[shard_idx];
//         let shard = shard.write();

//         // Atomically,
//         // - check if `obj` is already in the map
//         //   - if so, clone its `Arc` and return it
//         //   - if not, box it up, insert it, and return a clone
//         // This needs to be atomic (locking the shard) to avoid races with
// other thread,         // which could insert the same object between us
// looking it up and         // inserting it.

//         // FIXME: avoid double lookup/hashing by using raw entry API (once
// stable, or         // when hashbrown can be plugged into dashmap)
//         match shard.get_key_value(obj) {
//             Some((arc, _)) => Ok(Self { arc: arc.clone() }),
//             None => Err(shard),
//         }
//     }

//     fn alloc(arc: Arc<T>, mut shard: Guard<T>) -> Self {
//         let arc2 = arc.clone();

//         shard.insert(arc2, SharedValue::new(()));

//         Self { arc }
//     }
// }
