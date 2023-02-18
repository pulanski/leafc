/// The main entry point for the leafc compiler (i.e. the `leafc` binary).
///
/// Handles the command line arguments and then runs the driver or repl as
/// appropriate.
mod entry;

use entry::Leafc;

use miette::Result;
use std::process::ExitCode;

fn main() -> Result<ExitCode> {
    // Run the entry point for the compiler.
    Leafc::run()
}
