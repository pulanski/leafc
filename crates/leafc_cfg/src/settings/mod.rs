pub use self::{log::LogLevel, opt::OptLevel, target_triple::TargetTriple};

pub mod metadata;

pub mod log;
pub mod opt;
pub mod target_triple;

/// The **settings** for the compiler.
/// These are used to control the **behavior** of the compiler at runtime (e.g. the **optimization level**,
/// **verbosity level**, etc.). These are either **parsed** from the command-line arguments passed to the compiler
/// or defined in a user-defined **configuration file**.
// #[derive(Debug, Clone, PartialEq, Eq, Derivative)]
#[derive(Debug, Clone, PartialEq, Eq)]
// #[derivative(Default)]
pub struct Settings {
    /// The compiler's **version**.
    // #[derivative(Default(value = "crate_version()"))]
    pub version: String, // Semver object or git commit hash

    /// The **kinds** of output to emit from the compiler (e.g. the `AST`, `LLVM IR`, etc.).
    /// defaults to `vec![]`
    pub emit_kinds: Vec<EmitKind>,

    /// The **optimization level** to use when compiling the input file.
    /// defaults to `OptLevel::None`
    pub opt_level: OptLevel,

    /// The **verbosity level** of the compiler (e.g. `LogLevel::Info`, `LogLevel::Warn`, etc.).
    /// defaults to `LogLevel::Info`
    pub verbosity: LogLevel,

    /// The **target triple** to use when compiling the input file.
    /// defaults to `TargetTriple::Native`
    pub target_triple: TargetTriple,
}

/// The **kind of output** to emit from the compiler (e.g. the `AST`, `LLVM IR`, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmitKind {
    /// Emit the corresponding **`AST`** for the input file.
    Ast,

    /// Emit the corresponding **`LLVM IR`** for the input file.
    LlvmIr,

    /// Emit the corresponding **`object file`** for the input file.
    ObjectFile,

    /// Emit the corresponding **`LLVM bitcode`** for the input file.
    Bitcode,
}
