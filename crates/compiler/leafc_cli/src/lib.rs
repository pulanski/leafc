use std::path::PathBuf;

use clap::Parser;
use getset::Getters;
use leafc_cfg::settings::{
    meta::{
        pkg::EXE_NAME,
        version::LEAFC_VERSION,
    },
    EmitKind,
    LogLevel,
};

/// # **Leafc**, an Experimental Compiler
#[derive(Parser, Default, Debug, Getters, PartialEq, Eq, Hash)]
#[clap(name = EXE_NAME)]
#[clap(
    about = "Leafc, an experimental compiler",
    long_about = "An experimental compiler built on top of LLVM.",
    version = LEAFC_VERSION,
    author = "Josh Kersey <iopulanski@gmail.com>"
)]
#[clap(bin_name = EXE_NAME)]
#[getset(get = "pub")]
pub struct LeafcCli {
    /// Names of the source files to compile.
    pub sources: Vec<PathBuf>,

    /// The kinds of output to **print** from the compiler
    /// (e.g. the `AST`, `LLVM IR`, etc.). [default: []]
    ///
    /// This is useful for **debugging the compiler** and **visualizing the
    /// intermediate representations** of the source file.
    ///
    /// [Possible values: TokenStream, Ast, LlvmIr, ObjectFile, Bitcode, Asm]
    #[clap(short = 'd', long, value_parser, required = false)]
    pub debug: Vec<EmitKind>,

    /// This kinds of output to **emit** from the compiler
    /// (e.g. the `AST`, `LLVM IR`, etc.). [default: []]
    ///
    /// This is useful for **better understanding the compiler** and
    /// **visualizing the intermediate representations** of the source file
    /// in a concrete form.
    ///
    /// [Possible values: TokenStream, Ast, LlvmIr, ObjectFile, Bitcode, Asm]
    #[clap(short = 'e', long, value_parser, required = false)]
    pub emit: Vec<EmitKind>,

    /// Emit LLVM IR.
    /// [default: false]
    // Emit LLVM IR code generated from the middle-end of the compiler.
    //
    // This is useful for **debugging the middle-end** and **visualizing the
    // LLVM IR** code generated from the source file.
    // #[clap(short = 'r', long, value_parser, required = false, default_value_t = false)]
    // pub emit_llvm: bool,

    /// Step-through the compilation lifecycle.
    // Pause and wait for user input at the end of each phase in the compilation process.
    // [default: false]
    //
    // This is useful for incrementally debugging the compiler as a whole.
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

// TODO: refactor these

// Emit tokens.
// [default: false]
// This includes their **kind** and **span**.
// Emit tokens/words scanned from the lexical analysis phase of the compiler.
//
// This is useful for **debugging the lexer**.
// #[clap(short = 'l', long, value_parser, required = false, default_value_t =
// false)] pub debug_lexer: bool,

// Emit an AST.
// [default: false]
// Emit an AST created from the parsing phase of the compiler.
//
// This is useful for **debugging the parser** and **visualizing the
// syntactic structure** of the source file.
// #[clap(short = 'p', long, value_parser, required = false, default_value_t =
// false)] pub debug_parser: bool,
