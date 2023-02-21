#![doc = include_str!("../README.md")] // TODO: add this to shared docs (e.g. reference and source)
#![deny(
    missing_docs,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_results,
    bad_style,
    dead_code,
    improper_ctypes,
    noop_method_call,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    clippy::new_without_default,
    rustdoc::broken_intra_doc_links,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::complexity,
    clippy::style,
    clippy::perf,
    clippy::correctness,
    clippy::cargo,
    clippy::suspicious,
    rust_2018_idioms
)]
#![allow(
    dead_code, unused_variables, // TODO: temporary
    clippy::multiple_crate_versions, // required for transitive dependencies
)]

/// Defines a collection of utilities for working with the various
/// string types (e.g. `String`, `&str`, etc.).
pub mod string;

// pub mod fs;

/// Defines a collection of utilities for working with [`chrono`]
/// via more ergonomic APIs.
pub mod time;

/// Defines a collection of utilities for working with **terminal output**.
/// These utilities are used to **format** and **colorize** the output of
/// the compiler.
pub mod terminal;

/// Defines a number of **more idiomatic** APIs for working with positions
/// and spans found within the source code (e.g.
/// [`LineColumn`][crate::codemap::LineColumn],
/// [`Span`][crate::codemap::Span], etc.). These APIs are used to provide
/// **context** for error messages and to **highlight** the source code.
pub mod codemap;

/// Defines a collection of utilities for working with **regular expressions**.
/// These utilities are used for generating **random strings** that match a
/// given regular expression.
pub mod regex_gen;

/// Defines a collection of utilities for working with **collections** (e.g.
/// [`Vec`], [`HashMap`][crate::collections::HashMap], etc.).
pub mod collections;

pub use {
    codemap::{
        FileId,
        LineColumn,
        Location,
        Span,
        Spanned,
    },
    //  regex_gen::{
    //     RegexGen,
    //     RegexGenBuilder,
    // }, string::{
    //     StringExt,
    //     StringExtMut,
    // }, time::{
    //     DateTimeExt,
    //     DurationExt,
    //     NaiveDateTimeExt,
    //     NaiveTimeExt,
    //     OffsetDateTimeExt,
    //     TimeExt,
    // },};
    terminal::{
        horizontal_padding,
        vertical_padding,
    },
};
