use std::path::PathBuf;

use derivative::Derivative;
use derive_builder::Builder;

use crate::settings::target_triple::TargetTripleData;
use crate::settings::{emit::EmitKinds, LogLevel, OptLevel, TargetTriple};

/// Settings for **adjusting the behavior** of the compiler from the command
/// line. These are **parsed** from the command-line arguments passed to the
/// compiler.
///
/// This data structure is mostly used as a bridge between the `clap` crate and
/// the rest of the compiler in regards to configuring the compiler from the
/// command line.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Builder, Derivative)]
#[derivative(Default(new = "true"))]
pub struct CommandLineConfiguration {
    /// The **kinds** of output to emit from the compiler (e.g. the `AST`, `LLVM
    /// IR`, etc.). defaults to `vec![]`
    #[builder(default = "vec![]")]
    #[derivative(Default(value = "EmitKinds::default()"))]
    pub emit_kinds: EmitKinds,

    /// The **output directory** to use when emitting output from the compiler.
    /// defaults to `PathBuf::from("target")`
    #[builder(default = "PathBuf::from(\"target\")")]
    #[derivative(Default(value = "PathBuf::from(\"target\")"))]
    pub output_dir: PathBuf,

    /// The **optimization level** to use when compiling the input file.
    /// defaults to `OptLevel::None`
    #[builder(default = "OptLevel::None")]
    #[derivative(Default(value = "OptLevel::None"))]
    pub opt_level: OptLevel,

    /// The **verbosity level** of the compiler (e.g. `LogLevel::Info`,
    /// `LogLevel::Warn`, etc.). defaults to `LogLevel::Info`
    #[builder(default = "LogLevel::Info")]
    #[derivative(Default(value = "LogLevel::Info"))]
    pub verbosity: LogLevel,

    /// The **target triple** to use when compiling the input file.
    /// defaults to `TargetTriple::Native`
    // TODO: change this api to be more ergonomic
    #[builder(default = "TargetTriple::Native(TargetTripleData::default())")]
    #[derivative(Default(value = "TargetTriple::Native(TargetTripleData::default())"))]
    pub target_triple: TargetTriple,
}

#[cfg(test)]
mod cli_config_test_suite {
    use super::*;

    #[test]
    fn test_default() {
        let config = CommandLineConfiguration::default();

        assert_eq!(config.emit_kinds, EmitKinds::default());
        assert_eq!(config.output_dir, PathBuf::from("target"));
        assert_eq!(config.opt_level, OptLevel::None);
        assert_eq!(config.verbosity, LogLevel::Info);
        assert_eq!(config.target_triple, TargetTriple::Native(TargetTripleData::default()));
    }
}
