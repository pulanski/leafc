// use config::{Config, Environment, File};
// use miette::Result;
// use serde::Deserialize;

use self::{
    emit::EmitKinds,
    target_triple::TargetTripleData,
};
pub use self::{
    log::LogLevel,
    opt::OptLevel,
    target_triple::TargetTriple,
};
pub use emit::EmitKind;

/// Defines the **kinds** of output to emit from the compiler (e.g. the `AST`,
/// `LLVM IR`, etc.).
pub mod emit;

/// Defines the **meta** information for the compiler (e.g. the **version**,
/// **logo**, etc.).
pub mod meta;

/// Defines the **kinds** of output to emit from the compiler (e.g. the `AST`,
/// `LLVM IR`, etc.).
pub mod log;

/// Defines the **settings** used to **tune** the compiler's behavior (e.g. the
/// **log level**, **log topic**, **optimization level**, etc.) which can be set
/// from the command line (e.g. `leafc --log-level=debug`).
// pub mod command_line;

/// Defines the **optimization level** to use when compiling the input file.
pub mod opt;

/// Defines the vaious **settings** for the **REPL** (e.g. the **theme**,
/// **syntax highlighting**, etc.).
pub mod repl;

/// Defines the **target triple** to use when compiling the input file.
pub mod target_triple;

// TODO: singular settings struct (can be represented as a struct of structs via
// a toml file) contains batch compiler settings (e.g. optimization level), repl
// settings (i.e. theme, etc.), etc. settings can be passed via a local config
// file, or via the cli, or a user-level config file (e.g. ~/.leafc/config.
// toml). By default, the settings are set to the default values, and the
// precedence is as follows: cli > local config file > user-level config file >
// default values. The settings can be used to configure the behavior of the
// compiler. For example, the settings can be used to configure the compiler to
// emit LLVM IR, or to emit assembly code, or to emit object files, etc.

// when emit is turned on in the repl, the repl will emit the corresponding
// output for the input (e.g. the repl will emit the corresponding AST for the
// input, or the repl will emit the corresponding LLVM IR for the input, etc.).
// This can also be used to log the corresponding output for the input (e.g. the
// repl will log the corresponding AST for the input, or the repl will log the
// corresponding LLVM IR for the input, etc.) to a local log file.

/// The **settings** for the compiler.
/// These are used to control the **behavior** of the compiler at runtime (e.g.
/// the **optimization level**, **verbosity level**, etc.). These are either
/// **parsed** from the command-line arguments passed to the compiler or defined
/// in a user-defined **configuration file**.
// #[derive(Debug, Clone, PartialEq, Eq, Derivative)]
#[derive(Debug, Clone, PartialEq, Eq)]
// #[derivative(Default)]
pub struct LeafcSettings {
    /// The compiler's **version**.
    // #[derivative(Default(value = "crate_version()"))]
    pub version: String, // Semver object or git commit hash

    /// The **kinds** of output to emit from the compiler (e.g. the `AST`, `LLVM
    /// IR`, etc.). defaults to `vec![]`
    pub emit_kinds: EmitKinds,

    /// The **optimization level** to use when compiling the input file.
    /// defaults to `OptLevel::None`
    pub opt_level: OptLevel,

    /// The **verbosity level** of the compiler (e.g. `LogLevel::Info`,
    /// `LogLevel::Warn`, etc.). defaults to `LogLevel::Info`
    pub verbosity: LogLevel,

    /// The **target triple** to use when compiling the input file.
    /// defaults to `TargetTriple::Native`
    pub target_triple: TargetTriple,
}

impl Default for LeafcSettings {
    fn default() -> Self {
        Self::new()
    }
}

impl LeafcSettings {
    /// Initializes the **settings** for the compiler with the **default**
    /// values specified by the developers.
    ///
    /// # Returns
    ///
    /// A `Result` containing the **settings** for the compiler.
    pub fn new() -> LeafcSettings {
        LeafcSettings {
            version:       "0.1.0".to_string(),
            emit_kinds:    vec![],
            opt_level:     OptLevel::None,
            verbosity:     LogLevel::Info,
            target_triple: TargetTriple::Native(TargetTripleData::new()),
        }
    }
    //     let run_mode = env::var("RUN_MODE").unwrap_or_else(|_|
    // "development".into());

    //     let s = Config::builder()
    //         // Start off by merging in the "default" configuration file
    //         .add_source(File::with_name("examples/hierarchical-env/config/
    // default"))         // Add in the current environment file
    //         // Default to 'development' env
    //         // Note that this file is _optional_
    //         .add_source(
    //             File::with_name(&format!("examples/hierarchical-env/config/{}",
    // run_mode))                 .required(false),
    //         )
    //         // Add in a local configuration file
    //         // This file shouldn't be checked in to git
    //         .add_source(File::with_name("examples/hierarchical-env/config/local"
    // ).required(false))         // Add in settings from the environment (with
    // a prefix of APP)         // Eg.. `APP_DEBUG=1 ./target/app` would set the
    // `debug` key         .add_source(Environment::with_prefix("app"))
    //         // You may also programmatically change settings
    //         .set_override("database.url", "postgres://")?
    //         .build()?;

    //     // Now that we're done, let's access our configuration
    //     println!("debug: {:?}", s.get_bool("debug"));
    //     println!("database: {:?}", s.get::<String>("database.url"));

    //     // You can deserialize (and thus freeze) the entire configuration as
    //     s.try_deserialize()
    // }
}

// settings can be constructed from config files, cli args, or a combination of
// both in general, the precedence is as follows: cli > config file > default
// values to tackle this, we can have a `LeafcSettings` struct that contains all
// the settings and then we can have a `LeafcSettingsBuilder` struct that
// contains all the settings

// Mutates the compiler's **settings** from the command-line arguments passed to
// the compiler. This occurs after the **configuration file** has been parsed
// and the **settings** have been initialized to the **default values**.
//
//    mut again               mutate              init
//  command-line     <-     config file   <-    default values

// struct settings;

// pub fn with_cli_config(cli_config: &CommandLineConfig,&mut settings:
// LeafcSettings) -> Result<LeafcSettings> { pub fn (cli_config:
// &CommandLineConfig, LeafcSettings) -> Result<LeafcSettings> {     todo!()
// }
