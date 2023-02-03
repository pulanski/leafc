//! The leafc driver crate.
//!
//! This crate is the entry point for the compiler. It is responsible for
//! parsing the command line arguments, and then calling the appropriate
//! functions in the other crates to perform the compilation.
//!
//! The driver crate is also responsible for setting up the compiler's
//! logging system, and for handling the output of the compiler.
//!
#![deny(
    missing_docs,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
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
    // clippy::cargo,
    clippy::suspicious,
    rust_2018_idioms
)]
#![allow(
    clippy::multiple_crate_versions, // required for transitive dependencies
)]
#![feature(rustdoc_missing_doc_code_examples)]

use std::process::ExitCode;

use clap::Parser;
use derivative::Derivative;
use derive_builder::Builder;
use leafc_cfg::settings::EmitKind;
use leafc_errors::cli::CliError;
use log::{log, Level};
use miette::{IntoDiagnostic, Result};
use smol_str::SmolStr;
use tokio::fs::File;

// use leafc_cfg::settings::LogLevel;
use leafc_cli::LeafcCli;

/// The **leafc driver**. This is primary engine through which the compiler
/// is run.
///
/// This struct is responsible for orchestrating the various phases and subsystems
/// found within the compiler from **parsing the command line arguments**, to
/// **building an abstract syntax tree**, to **performing type checking** and
/// **code generation**.
///
/// The driver is also responsible for setting up the compiler's logging system,
/// and for handling the output of the compiler.
///
/// // TODO: Create a `DriverBuilder` struct to allow for customizing the driver.
/// via some configuration options.
///
/// # Examples
///
/// ```rust,no_run
/// use leafc_driver::LeafcDriver;
///
/// // Run a new driver with the default configuration.
/// LeafcDriver::run();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Derivative, Builder)]
#[derivative(Default(new = "true"))]
pub struct LeafcDriver {
    /// The driver's version.
    #[derivative(Default(value = "\"0.1.0\".into()"))]
    version: SmolStr,

    /// The kinds of output to emit.
    // #[derivative(Default(value = "vec![EmitKind::Ast]"))]
    // have this default to the default defined in settings.rs within the leafc_cfg crate
    #[derivative(Default(value = "vec![]"))]
    emit_kinds: Vec<EmitKind>,
}

impl LeafcDriver {
    /// The driver's version.
    pub const VERSION: &'static str = "0.1.0";

    /// Runs the **leafc driver** and returns an exit code indicating whether
    /// the compilation was successful or not.
    ///
    /// This function is the primary entry point for the compiler. It is
    /// responsible for parsing the command line arguments, and then calling the
    /// appropriate functions in the other crates to perform the compilation.
    ///
    /// The driver is also responsible for setting up the compiler's logging
    /// system, and for handling the output of the compiler.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use leafc_driver::LeafcDriver;
    ///
    /// LeafcDriver::run();
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the driver fails to run. This
    /// could be due to a number of reasons, including:
    ///
    /// - The driver failed to **parse the command line arguments**.
    /// - The driver failed to **parse the input file**.
    /// - The driver failed to **perform type checking**, or **perform
    ///  name resolution**.
    /// - The driver failed to **generate code.**
    /// - The driver failed to **emit the output**.
    ///
    /// In general, the various subsystems within the compiler will return
    /// errors if they fail to perform their tasks. The driver will then
    /// propagate these errors up to the caller via the `Result` type containing
    /// the corresponding `ExitCode` indicating the reason for the failure for
    /// the particular error.
    pub async fn run() -> Result<ExitCode> {
        let cli = LeafcCli::parse();

        // initialize the logging system
        leafc_log::init(cli.verbosity)?;

        // iterate over the files to compile
        for file in cli.sources {
            let _f = File::open(file.clone())
                .await
                .into_diagnostic()
                .map_err(|error| CliError::FileNotFound(format!("{file:?}: {error}").into()))?;
            eprintln!("Compiling file: {file:?} ...");
        }

        // log!(LogLevel::Info.into(), "leafc driver version: {}", Self::VERSION);
        log!(Level::Info, "leafc driver version: {}", Self::VERSION);
        // leafc_log!(LogLevel::Info, "leafc driver version: {}", Self::VERSION);

        Ok(ExitCode::SUCCESS)
    }
}
