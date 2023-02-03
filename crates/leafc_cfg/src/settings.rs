pub mod target_triple;
pub mod version;
// pub mod emit;
// pub mod log;
// pub mod opt;

use derivative::Derivative;
use strum_macros::Display;

use clap::ValueEnum;

use target_triple::TargetTriple;

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

/// The **verbosity level** of the compiler. This is used to control the **amount of logging** emitted by the
/// compiler.
/// defaults to `LogLevel::Info`
///
/// In general, when setting the log level of interest (e.g. `LogLevel::Debug`), this will cause all less important
/// log levels to be ignored
///
/// The following diagram illustrates how this works.
///
/// `Fatal` <- `Error` <- `Warn` <- `Info` <- `Debug` <- `Trace`
///
/// So, if the log level is set to `LogLevel::Info`, then only `LogLevel::Info`, `LogLevel::Warn`, and
/// `LogLevel::Error` messages will be emitted.
///
/// # Examples
///
/// ```rust
/// use leafc_cfg::settings::LogLevel;
///
/// assert!(LogLevel::Trace < LogLevel::Debug);
/// assert!(LogLevel::Debug < LogLevel::Info);
/// assert!(LogLevel::Info < LogLevel::Warn);
/// assert!(LogLevel::Warn < LogLevel::Error);
/// assert!(LogLevel::Error < LogLevel::Fatal);
/// ```
#[derive(Debug, Default, Display, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, ValueEnum)]
pub enum LogLevel {
    /// The **most fine-grained** level for logging. This is the **most verbose** level, and is useful for
    /// annotating the compiler's internal state at each step in the compilation process (e.g. the `AST`, `LLVM IR`,
    /// etc.). This can be used to better understand **where performance bottlenecks are occurring**.
    Trace,

    /// The **second most fine-grained** level for logging. This is less granular than [`LogLevel::Trace`], but is
    /// still useful for annotating the compiler's internal state at each step in the compilation process. This can
    /// be useful for **incrementally debugging** the compiler state as a whole.
    Debug,

    /// The **default** level for logging. This is **less granular** than [`LogLevel::Debug`], but is still useful
    /// for annotating the compiler's internal state at each step in the compilation process and is used to present
    /// **high-level information** about the compiler's progress to the user.
    #[default]
    Info,

    /// The level for displaying **warnings**. This is a **higher-level** of logging than [`LogLevel::Info`], and is
    /// used for annotating unexpected behavior in the compiler (e.g. a bug in the compiler, or a bug in the user's
    /// code).
    Warn,

    /// The level for displaying **errors**. This is a **higher-level** of logging than [`LogLevel::Warn`], and is
    /// used for annotating unexpected behavior within a subsystem of the compiler. This is also used to annotate a
    /// _blast radius_ for **errors** that occur during the compilation process
    Error,

    /// The level for displaying **fatal errors**. This is a **higher-level** of logging than [`LogLevel::Error`],
    /// and is used for annotating **fatal behavior** that **prevents the compiler from continuing**.
    Fatal,
}

/// The **category** of a log message (e.g. `Lexer`, `Parser`, etc.). This is used to control the **amount of
/// logging** emitted by the compiler and is used to **filter** log messages by their **category** (e.g. only
/// display `Lexer` messages).
pub enum LogTopic {
    /// A log message that is associated with the **`Lexer`**.
    Lexer,
    /// A log message that is associated with the **`Parser`**.
    Parser,
    /// A log message that is associated with the **`TypeChecker`**.
    TypeCheck,
    /// A log message that is associated with **`CodeGen`**.
    CodeGen,
}

impl From<LogLevel> for log::Level {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Trace => log::Level::Trace,
            LogLevel::Debug => log::Level::Debug,
            LogLevel::Info => log::Level::Info,
            LogLevel::Warn => log::Level::Warn,
            LogLevel::Error => log::Level::Error,
            LogLevel::Fatal => log::Level::Error,
        }
    }
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

/// The **optimization level** to use when compiling the input file.
/// defaults to `OptLevel::None`
///
/// In general, there is an unavoidable tradeoff between compiler optimization and compile times. In
/// order to produce the most well-optimized executable, we pay the cost of higher compilation times.
/// However, this is not always desirable. For example, during development, we may want to compile our
/// code as quickly as possible, and we are not concerned with the performance of the executable. In
/// this case, we can use the [`OptLevel::None`] optimization level.
///
/// # Examples
///
/// ```rust
/// use leafc_cfg::settings::OptLevel;
///
/// // The default optimization level is `OptLevel::None` (i.e. no optimizations).
/// // The higher the optimization level, the more optimizations are performed.
/// assert!(OptLevel::None < OptLevel::O1);
/// assert!(OptLevel::O1 < OptLevel::O2);
/// assert!(OptLevel::O2 < OptLevel::O3);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum OptLevel {
    /// **No optimizations** are performed. This is the default optimization level.
    /// Useful for **debugging purposes** during development.
    None,

    /// **Basic optimizations passes** are performed.
    O1,

    /// More advanced optimizations are performed in combination to those performed in [`OptLevel::O1`].
    O2,

    /// The most advanced optimizations are performed in combination to those performed in [`OptLevel::O2`].
    /// This is the **most optimized** level.
    /// Useful for **production**.
    O3,
}
