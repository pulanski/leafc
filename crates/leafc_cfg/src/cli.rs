use derive_builder::Builder;

use crate::settings::target_triple::TargetTripleData;
use crate::settings::{emit::EmitKinds, EmitKind, LogLevel, OptLevel, TargetTriple};

/// Settings for **adjusting the behavior** of the compiler from the command line.
/// These are **parsed** from the command-line arguments passed to the compiler.
///
/// This data structure is mostly used as a bridge between the `clap` crate and the rest of the compiler
/// in regards to configuring the compiler from the command line.
///
/// # Examples
///
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Builder)]
pub struct CommandLineConfig {
    /// The **kinds** of output to emit from the compiler (e.g. the `AST`, `LLVM IR`, etc.).
    /// defaults to `vec![]`
    #[builder(default = "vec![]")]
    pub emit_kinds: EmitKinds,

    /// The **optimization level** to use when compiling the input file.
    /// defaults to `OptLevel::None`
    #[builder(default = "OptLevel::None")]
    pub opt_level: OptLevel,

    /// The **verbosity level** of the compiler (e.g. `LogLevel::Info`, `LogLevel::Warn`, etc.).
    /// defaults to `LogLevel::Info`
    #[builder(default = "LogLevel::Info")]
    pub verbosity: LogLevel,

    /// The **target triple** to use when compiling the input file.
    /// defaults to `TargetTriple::Native`
    // TODO: change this api to be more ergonomic
    #[builder(default = "TargetTriple::Native(TargetTripleData::default())")]
    pub target_triple: TargetTriple,
}

impl CommandLineConfig {
    pub fn new(
        emit_kinds: Vec<EmitKind>,
        opt_level: OptLevel,
        verbosity: LogLevel,
        target_triple: TargetTriple,
    ) -> CommandLineConfig {
        CommandLineConfig { emit_kinds, opt_level, verbosity, target_triple }
    }
}
