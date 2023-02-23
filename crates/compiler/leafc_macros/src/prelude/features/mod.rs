// #[macro_use]
// mod serde;

mod syntax;

// TODO: get this working more

/// [`serde`] feature declaration, which is used to enable the
/// [`Serialize`] and [`Deserialize`] traits.
// #[macro_export]
// macro_rules! SERDE_FEATURE_USE_DECL {
//     // take a list of strings like `"json", "yaml"`, convert them to types, and then use that
// type     // in the `use` statement.
//     () => {
//         // check if the "json" feature is enabled
//         #[cfg(all(feature = "serde", feature = "json"))]
//         {
//             // use the `serde_json` crate
//             use serde_json::*;
//         }

//         #[cfg(all(feature = "serde", feature = "yaml"))]
//         {
//             // use the `serde_yaml` crate
//             use serde_yaml::*;
//         }

//         #[cfg(all(feature = "serde", feature = "bytes"))]
//         {
//             // use the `serde_bytes` crate
//             use serde_bytes::*;
//         }

//         SERDE_FEATURE_USE_DECL_BASE!();
//     };
// }

#[macro_export]
macro_rules! SERDE_FEATURE_USE_DECL_BASE {
    () => {
        #[cfg(feature = "serde")]
        use serde::{
            Deserialize,
            Serialize,
        };
    };
}

#[macro_use]
mod allocative;

/// Macro for declaring the (**optional**) features used by the various
/// [`leafc`] crates.
///
/// Declares the following features:
///
/// - [`serde`] - used to enable the [`Serialize`] and [`Deserialize`] traits.
///   (useful for serialization and deserialization with other formats and
///   tools)
/// - [`allocative`] - used to enable the [`Allocative`] trait (useful for
///   performance optimizations by inspecting memory allocations).
// #[macro_export]
// macro_rules! LEAFC_FEATURE_USE_DECLS {
//     () => {
//         SERDE_FEATURE_USE_DECL!();
//         ALLOCATIVE_FEATURE_DECL!();
//     };
// }

// #[cfg(feature = "serde")]
// #[cfg(test)]
// mod serde_smoke_tests {
//     #[test]
//     fn test_leafc_feature_use_decls() {
//         LEAFC_FEATURE_USE_DECLS!();

//         #[derive(Serialize, Deserialize)]
//         struct Foo {}
//     }

//     #[test]
//     fn test_serde_feature_use_decl() {
//         LEAFC_FEATURE_USE_DECLS!();

//         #[derive(Serialize, Deserialize)]
//         struct Foo {}
//     }
// }

// TODO: add macro for `multithreaded` use case. Macro should take a list of
// various dependencies that are required for the `multithreaded` feature.
// (e.g. `std::sync::Arc`, `parking_lot::Mutex`, etc.)
//
// Example:
//
// MULTITHREADED_FEATURE_USE_DECLS!(std::sync::Arc, parking_lot::Mutex);
//
// This should expand to:
//
// #[cfg(feature = "multithreaded")]
// {
//     use std::sync::Arc;
//     use parking_lot::Mutex;
// }
#[macro_export]
macro_rules! MULTITHREADED_FEATURE_USE_DECLS {
    // take a list of dependencies that are required for the `multithreaded`
    // feature.
    ($($dep:ty),*) => {
        MULTITHREADED_FEATURE_DECL!($($dep),*);
    };
}

#[macro_export]
macro_rules! MULTITHREADED_FEATURE_DECL {
    // take a a string like `"std::sync::Arc", "parking_lot::Mutex"`, convert
    // it to a list of types, and then use those types in the `use` statement.

    // NOTE: this is a bit of a hack, but it works. The `use` statement is
    // generated at compile time, so it doesn't matter that the types don't

    // exist at compile time. The types are only used to generate the `use`

    // statement, and the `use` statement is only used to generate the
    // `#[cfg(feature = "multithreaded")]` block.

    ($($dep:ty),*) => {
        #[cfg(feature = "multithreaded")]
        {
            use $($dep),*;
        }
    };
}

// #[cfg(feature = "multi-threaded")]
// #[cfg(test)]
// mod multithreaded_smoke_tests {
//     #[test]
//     fn test_leafc_feature_use_decls() {
//         LEAFC_FEATURE_USE_DECLS!();
//     }

//     #[test]
//     fn test_multithreaded_feature_use_decls() {
//         // MULTITHREADED_FEATURE_USE_DECLS!(std::sync::Arc,
// parking_lot::Mutex);         // let _ = Arc::new(());
//         // let a = u128::MAX;

//         // let b =

//         // let _ = Mutex::new(());
//     }
// }
