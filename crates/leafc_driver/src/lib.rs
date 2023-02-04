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

use std::{fs::File, process::ExitCode};

use clap::Parser;
use derivative::Derivative;
use derive_builder::Builder;
use leafc_cfg::settings::EmitKind;
use leafc_errors::cli::CliError;
// use leafc_repl::LeafcRepl;
// use log::{log, Level};
use miette::{IntoDiagnostic, Result};
use smol_str::SmolStr;

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

    /// Runs the **top level driver** for **leafc** and returns an exit code indicating whether
    /// the compilation was successful or not.
    ///
    /// This function is the primary entry point for the compiler. It is
    /// responsible for parsing the command line arguments, and then starting
    /// the appropriate pipeline to perform the compilation based on whether the
    /// compiler is running in **repl mode** or not.
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
    pub fn run() -> Result<ExitCode> {
        // parse the command line arguments
        let cli = LeafcCli::parse();

        // initialize the logging system
        leafc_log::init(cli.verbosity)?;

        // log the driver's version
        // leafc_log::utils::version();

        // run the driver (either the repl or the batch compiler)
        if cli.sources.is_empty() {
            // run the repl
            Self::repl_run(&cli)
        } else {
            // run the batch compiler
            Self::batch_run(&cli)
        }
    }

    /// Runs the **leafc driver** with the given command line arguments in **batch
    /// compilation mode** and returns an exit code indicating whether the compilation
    /// was successful.
    ///
    /// This function is the primary entry point for the compiler when compiling
    /// source files in the typical batch processing model. It takes the command
    /// line arguments and then calls the appropriate functions in the other crates
    /// to perform the standard **batch compilation pipeline** (similar to running
    /// `rustc`, or `gcc`).
    ///
    /// // TODO: abstract this out into settings instead of passing in the cli
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use leafc_driver::LeafcDriver;
    ///
    /// // Execute the compilation pipeline on the user's input files.
    /// LeafcDriver::batch_run(LeafcCli::parse());
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
    /// name resolution**.
    ///
    /// In general, the various subsystems within the compiler will return
    /// errors if they fail to perform their tasks. The driver will then
    /// propagate these errors up to the caller via the `Result` type containing
    /// the corresponding `Error` indicating the reason for the failure for
    /// the particular error. These errors can be found in the `leafc_errors`
    /// crate.
    pub fn batch_run(cli: &LeafcCli) -> Result<ExitCode> {
        // iterate over the files to compile
        for file in &cli.sources {
            let _f = File::open(file.clone())
                .into_diagnostic()
                .map_err(|error| CliError::FileNotFound(format!("{file:?}: {error}").into()))?;
            // leafc_log!(LogLevel::Info, "Compiling {} ...", file); // TODO: add this with spinners
            // TODO: abstract the api to look like this
            // leafc_log::utils::compiling_file(file);
            eprintln!("Compiling file: {file:?} ...");
        }

        Ok(ExitCode::SUCCESS)
    }

    /// Runs the **leafc driver** with the given command line arguments
    /// in **repl mode** and returns an exit code indicating whether the
    /// creation of the repl was successful.
    ///
    /// This function is the primary entry point for the compiler when running
    /// in **repl mode**. It takes the command line arguments and then calls
    /// the appropriate functions in the other crates to perform the standard
    /// **repl compilation pipeline** (similar to using the `swift` repl or
    /// the `julia` repl).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use leafc_driver::LeafcDriver;
    ///
    /// // Execute the compilation pipeline in repl mode.
    /// LeafcDriver::repl_run(LeafcCli::parse());
    /// LeafcDriver::repl_run(LeafcCli::new());
    ///
    /// // TODO change api to new method for things that rely on the Cli,
    /// the new method encapsulates the parse method
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the driver fails to run. This
    /// is caused by the repl failing to initialize.
    ///
    /// In general, the repl will return errors if it fails to perform its
    /// tasks. The driver will then propagate these errors up to the caller
    /// via the `Result` type containing the corresponding `Error` indicating
    /// the reason for the failure for the particular error. These errors can
    /// be found in the `leafc_errors` crate.
    #[allow(clippy::needless_pass_by_value)] // TODO: for now
    pub fn repl_run(cli: &LeafcCli) -> Result<ExitCode> {
        // let _repl = LeafcRepl::new(cli);
        eprintln!("CLI: {cli:?}");
        // TODO: add repl
        // run the repl
        // repl.run(LeafcRepl::new(cli))?;

        leafc_repl::run();

        // settings is from the settings crate
        // cli is used to update global LeafcSettings struct
        // and we can use From<LeafcSettings> for just the ReplSettings
        // leafc_repl::LeafcRepl::run(settings)?; // TODO change to something along the lines of this api

        Ok(ExitCode::SUCCESS)
    }
}

/// The interface required to run the driver.
/// This is used to abstract away the driver's implementation details.
/// It takes the settings for the driver and runs the driver. These settings
/// are typically provided by the command line arguments, but can also be
/// provided by other means (config files, etc.).
///
/// The settings are used to configure the behavior of the compiler. For
/// example, the settings can be used to configure the compiler to emit
/// LLVM IR, or to emit assembly code, or to emit object files, etc.
/// The settings can also be used to configure the compiler to emit
/// diagnostics to the console, or to emit diagnostics to a file, etc.
/// One last use case for the settings is to configure the compiler to emit
/// debug information, or to emit debug information in a specific format,
/// etc.
trait Runnable {
    // /// Runs the driver and returns an exit code indicating whether the
    // /// compilation was successful.
    // ///
    // /// This can be used to run the driver in a custom way, for example
    // /// in a REPL vs. a batch compiler.
    // fn run(&self, settings: &LeafcSettings) -> Result<ExitCode>;
    // fn run(&self) -> Result<ExitCode>;
}

// TODO: create a From<LeafcSettings> for ReplSettings
// impl From<LeafcSettings> for ReplSettings {
