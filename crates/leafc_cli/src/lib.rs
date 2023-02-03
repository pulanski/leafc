use std::path::PathBuf;

use clap::Parser;
use leafc_cfg::settings::log::LogLevel;

/// # **Leafc**, an Experimental Compiler
#[derive(Parser, Default, Debug)]
#[clap(name = "leafc")]
#[clap(
    about = "Leafc, an experimental compiler",
    long_about = "An experimental compiler built on top of LLVM.",
    version = "0.1.0",
    author = "Josh Kersey <iopulanski@gmail.com>"
)]
#[clap(bin_name = "leafc")]
#[clap(version)]
pub struct LeafcCli {
    /// Name of the **source files** to compile. This is the only
    /// required argument. (e.g. `foo.leaf`).
    #[clap(required = true)]
    pub sources: Vec<PathBuf>,

    /// Emit tokens/words scanned from the lexing phase of the compiler along with their spans and
    /// lexemes and other useful metadata.
    /// [default: false]
    ///
    /// This is useful for **debugging the lexer**.
    #[clap(short = 's', long, value_parser, required = false, default_value_t = false)]
    pub emit_scan: bool,

    /// Emit an AST created from the parsing phase of the compiler.
    /// [default: false]
    ///
    /// This is useful for **debugging the parser** and **visualizing the
    /// syntactic structure** of the source file.
    #[clap(short = 'p', long, value_parser, required = false, default_value_t = false)]
    pub emit_parse: bool,

    /// Emit LLVM IR code generated from the middle-end of the compiler.
    /// [default: false]
    #[clap(short = 'l', long, value_parser, required = false, default_value_t = false)]
    pub emit_llvm: bool,

    /// Pause and wait for user input at the end of each phase in the compilation process.
    /// [default: false]
    ///
    /// This is useful for incrementally debugging the compiler as a whole.
    #[clap(short = 'i', long, value_parser, required = false, default_value_t = false)]
    pub interactive: bool,

    /// The verbosity level of the compiler.
    /// [default: info]
    #[clap(short = 'v', long, required = false, value_enum, default_value = "info")]
    pub verbosity: LogLevel,
}

impl LeafcCli {
    /// Parse the command line arguments
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
