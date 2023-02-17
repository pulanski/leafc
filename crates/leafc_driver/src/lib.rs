//! The leafc driver crate.
//!
//! This crate is the entry point for the compiler. It is responsible for
//! parsing the command line arguments, and then calling the appropriate
//! functions in the other crates to perform the compilation.
//!
//! The driver crate is also responsible for setting up the compiler's
//! logging system, and for handling the output of the compiler.
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

use std::{
    path::PathBuf,
    process::ExitCode,
    time::Duration,
};

use derivative::Derivative;
use derive_builder::Builder;
use getset::{
    Getters,
    MutGetters,
    Setters,
};
use indicatif::{
    ProgressBar,
    ProgressStyle,
};
use leafc_cfg::{
    cli::CommandLineConfiguration,
    settings::{
        emit::EmitKinds,
        EmitKind,
    },
};
use leafc_cli::LeafcCli;
use leafc_errors::{
    cli::CliError,
    driver::DriverError,
};
use leafc_lexer::lexer::TokenStream;
use miette::{
    IntoDiagnostic,
    Result,
};
use owo_colors::OwoColorize;
use smol_str::SmolStr;

/// The **leafc driver**. This is primary engine through which the compiler
/// is run.
///
/// This struct is responsible for orchestrating the various phases and
/// subsystems found within the compiler from **parsing the command line
/// arguments**, to **building an abstract syntax tree**, to **performing type
/// checking** and **code generation**.
///
/// The driver is also responsible for setting up the compiler's logging system,
/// and for handling the output of the compiler.
///
/// // TODO: Create a `DriverBuilder` struct to allow for customizing the
/// driver. via some configuration options.
///
/// # Examples
///
/// ```rust,no_run
/// use leafc_driver::LeafcDriver;
///
/// // TODO: add an example of how to use the driver
/// // Run a new driver with the default configuration.
/// // LeafcDriver::compile();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Derivative, Builder, Getters, Setters, MutGetters)]
#[derivative(Default(new = "true"))]
pub struct LeafcDriver {
    /// The driver's version.
    #[derivative(Default(value = "\"0.1.0\".into()"))]
    #[getset(get = "pub")]
    version: SmolStr,

    /// The kinds of output to emit.
    // #[derivative(Default(value = "vec![EmitKind::Ast]"))]
    // have this default to the default defined in settings.rs within the leafc_cfg crate
    #[derivative(Default(value = "vec![]"))]
    #[getset(get = "pub", get_mut = "pub")]
    emit_kinds: EmitKinds,
}

impl LeafcDriver {
    /// **Compiles** the given source code, returning a result indicating
    /// whether the compilation was successful.
    ///
    /// # Errors
    ///
    /// If the driver fails to run the compilation pipeline, this function will
    /// return an error indicating the cause of the failure to compile.
    pub fn compile(&self, text_source: &str, lossless: bool) -> Result<()> {
        // lex the source code and produce a token stream
        let tokens = TokenStream::new(text_source, lossless);

        // if we are emitting the tokens, then log them
        if self.emit_kinds().contains(&EmitKind::TokenStream) {
            log::info!("{tokens}");
        }

        Ok(())
    }

    /// TODO: document
    pub fn apply_repl_settings(&mut self, settings: &CommandLineConfiguration) {
        self.emit_kinds_mut().clear();
        self.emit_kinds_mut().extend(settings.emit_kinds.clone());
    }
}

/// Runs the **leafc driver** with the given command line arguments in **batch
/// compilation mode** and returns an exit code indicating whether the
/// compilation was successful.
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
/// use clap::Parser;
/// use leafc_cli::LeafcCli;
/// use leafc_driver::LeafcDriver;
///
/// // Execute the compilation pipeline on the user's input files.
/// leafc_driver::batch_run(&LeafcCli::parse());
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
    // create a new driver
    let driver = LeafcDriver::new();

    // create a progress bar
    let pb = match cli.sources.len() {
        0..=1 => ProgressBar::hidden(),
        _ => ProgressBar::new(cli.sources.len() as u64),
    };

    // iterate over the files to compile
    for file in &cli.sources {
        let filename = get_filename(file)?;

        // leafc_log::compiling_file(filename); // TODO: update this to use the new
        // logger

        let style =
            ProgressStyle::default_bar().template("{spinner:.green} {msg}").map_err(|error| {
                DriverError::Initialization(
                    format!("failed to set progress bar style: {error}").into(),
                )
            })?;

        let progress_style = style.tick_strings(&["▹▹▹", "▸▹▹", "▹▸▹", "▹▹▸", "▪▪▪▪▪"]);

        let pb_spinner = ProgressBar::new_spinner();
        pb_spinner.enable_steady_tick(Duration::from_millis(120));
        pb_spinner.set_style(progress_style);

        // ProgressStyle::with_template("{spinner:.blue} {msg}").map_err(|error| {)
        // For more spinners check out the cli-spinners project:
        // https://github.com/sindresorhus/cli-spinners/blob/master/spinners.json
        // .tick_strings(&["▹▹▹▹▹", "▸▹▹▹▹", "▹▸▹▹▹", "▹▹▸▹▹", "▹▹▹▸▹", "▹▹▹▹▸",
        // "▪▪▪▪▪"]), );
        pb_spinner.set_message(format!("Compiling {}", filename.bright_green().bold()));

        // read the file into a string
        let text_source =
            std::fs::read_to_string(file.clone()).into_diagnostic().map_err(|error| {
                CliError::FileNotFound(
                    format!("{:?}: {}", file.green(), error.bright_yellow()).into(),
                )
            })?;

        // compile the file
        driver.compile(
            &text_source,
            false, /* we don't need full fidelity representation of the source */
        )?;
        pb_spinner.finish_with_message(format!("Compiled {}", filename.bright_green().bold()));
        pb.inc(1);
    }

    pb.finish_with_message("done");

    std::thread::sleep(std::time::Duration::from_secs(1));

    Ok(ExitCode::SUCCESS)
}

fn get_filename(file: &PathBuf) -> Result<&str> {
    let filename = file
        .file_name()
        .ok_or(CliError::FileNotFound(
            format!("{:?}: {}", file.green(), "File name terminates in `..`".bright_yellow())
                .into(),
        ))?
        .to_str()
        .ok_or(CliError::FileNotFound(
            format!("{:?}: {}", file.green(), "File name is not valid unicode".bright_yellow())
                .into(),
        ))?;

    Ok(filename)
}
