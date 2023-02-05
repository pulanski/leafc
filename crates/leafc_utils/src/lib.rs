/// Defines a collection of utilities for working with the various `String` types (e.g. `String`, `&str`, etc.).
pub mod string;

// pub mod fs;

/// Defines a collection of utilities for working with [`chrono`]
/// via more ergonomic APIs.
pub mod time;

pub mod terminal;

pub mod location;

pub use terminal::{horizontal_padding, vertical_padding};
