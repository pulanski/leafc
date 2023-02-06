use strum_macros::Display;

use clap::ValueEnum;

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
#[derive(Debug, Default, Display, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, ValueEnum, Hash)]
pub enum LogLevel {
    /// The most fine-grained level for logging.
    //
    // The **most fine-grained** level for logging. This is the **most verbose** level, and is useful for
    // annotating the compiler's internal state at each step in the compilation process (e.g. the `AST`,
    // `LLVM IR`, tc.). This can be used to better understand **where performance bottlenecks are
    // occurring**.
    Trace,

    /// The second most fine-grained level for logging (e.g. less granular than [`LogLevel::Trace`]).
    //
    // but is still useful for annotating the compiler's internal state at each step in the compilation process.
    // This can be useful for **incrementally debugging** the compiler state as a whole.
    Debug,

    /// Displays default information about the compiler's progress.
    //
    // The **default** level for logging.
    //
    // This is **less granular** than [`LogLevel::Debug`], but is still useful
    // for annotating the compiler's internal state at each step in the compilation process
    // and is used to present **high-level information** about the compiler's progress to the user.
    #[default]
    Info,

    /// Display warnings that may occur.
    //
    // The level for displaying **warnings**. This is a **higher-level** of logging than [`LogLevel::Info`], and is
    // used for annotating unexpected behavior in the compiler (e.g. a bug in the compiler, or a bug in the user's
    // code).
    Warn,

    /// Display errors that may occur.
    //
    // The level for displaying **errors**. This is a **higher-level** of logging than [`LogLevel::Warn`], and is
    // used for annotating unexpected behavior within a subsystem of the compiler. This is also used to annotate a
    // _blast radius_ for **errors** that occur during the compilation process
    Error,

    /// Display fatal errors that prevent the compiler from continuing.
    //
    // This is a **higher-level** of logging than [`LogLevel::Error`],
    // and is used for annotating **fatal behavior** that **prevents the compiler from continuing**.
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
    /// A log message that is associated performance **metrics**.
    Performance,
}

/// For converting a [`LogLevel`] into a [`log::Level`].
/// This is used to convert the [`LogLevel`] into a [`log::Level`] so that it can be used by the `log` crate.
/// meaning that the [`LogLevel`] can be used to control the **amount of logging** emitted by the compiler.
/// TODO: This is a **temporary** solution until the `leafc_log` crate is enhanced to support the `info!`, `warn!`, etc. macros
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
