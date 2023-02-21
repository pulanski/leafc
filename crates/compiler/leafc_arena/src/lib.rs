//! # A featureful arena allocator.
//!
//! This arena allocator is designed to be used in a compiler. It is
//! designed to be fast, and to have a small memory footprint. It is
//! also designed to be able to handle large allocations, and to
//! support deallocation of individual objects as well as the entire
//! arena at once
//!
//! "Deallocation" or "Dropping" is very fast, as it simply resets the
//! internal pointerto the start of the arena.
//!
//!
//! ## Features
//!
//! - Fast allocation of objects
//! - Small memory footprint
//! - Supports large allocations
//! - Supports deallocation of individual objects
//! - Supports **fast** deallocation of the entire arena at once
//! - Supports **type-safe** indices into the arena (see `ArenaIdx`)
//! - Supports **type-safe** iteration over the arena (see `ArenaIter`)
//! - Works with in a **multi-threaded** environment (e.g. `rayon`, `crossbeam`,
//!   etc.)
//! - TODO: Supports **custom allocators** (e.g. `jemalloc`, `mimalloc`, etc.)
//!
//! The arena is designed to be used in a **very-simple** "stack-like" fashion.
//! You allocate objects on the arena. When the arena goes out of scope,
//! all objects allocated on the arena are "dropped" at once.
//!
//! The arena also has a `clear` method, which can be used to drop all
//! objects allocated on the arena, but keep the arena itself. This
//! can be used to "reset" the arena to its initial state.
//!
//! NOTE: This reset is **very fast**. It simply resets the internal
//! pointer to the start of the arena. This means that the arena can
//! be used in a "stack-like" fashion, where you allocate objects on
//! the arena, and then "clear" the arena when you are done with those
//! objects, but still keep the arena itself.
//!
//! That's the core idea, the rest is just details along with some
//! optimizations and a few enchancements to overall API.

// pub struct Arena<T> {
//     data: Vec<T>,
// }

// use std::collections::VecDeque;

// pub struct Arena<T> {
//     data: VecDeque<T>,
//     size: usize,
// }
