pub use self::{log::LogLevel, opt::OptLevel, target_triple::TargetTriple};

pub mod meta;

pub mod log;
pub mod opt;
pub mod target_triple;

// TODO: singular settings struct (can be represented as a struct of structs via a toml file)
// contains batch compiler settings (e.g. optimization level), repl settings (i.e. theme, etc.), etc.
// settings can be passed via a local config file, or via the cli, or a user-level config file
// (e.g. ~/.leafc/config.toml). By default, the settings are set to the default values, and the
// precedence is as follows: cli > local config file > user-level config file > default values.
// The settings can be used to configure the behavior of the compiler. For example, the settings
// can be used to configure the compiler to emit LLVM IR, or to emit assembly code, or to emit
// object files, etc.

// when emit is turned on in the repl, the repl will emit the corresponding output for the input
// (e.g. the repl will emit the corresponding AST for the input, or the repl will emit the corresponding
// LLVM IR for the input, etc.). This can also be used to log the corresponding output for the input
// (e.g. the repl will log the corresponding AST for the input, or the repl will log the corresponding
// LLVM IR for the input, etc.) to a local log file.

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
