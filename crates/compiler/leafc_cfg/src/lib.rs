pub use salsa;

rust_i18n::i18n!("locales");

#[salsa::jar(db = Db)]
pub struct Jar(
    // crate::compile::compile,
    // crate::ir::SourceProgram,
    // crate::ir::Program,
    // crate::ir::VariableId,
    // crate::ir::FunctionId,
    // crate::ir::Function,
    // crate::ir::Diagnostics,
    // crate::ir::Span,
    // crate::parser::parse_statements,
    // crate::type_check::type_check_program,
    // crate::type_check::type_check_function,
    // crate::type_check::find_function,
);

// ANCHOR: jar_db
pub trait Db: salsa::DbWithJar<Jar> {}
// ANCHOR_END: jar_db

// ANCHOR: jar_db_impl
impl<DB> Db for DB where DB: ?Sized + salsa::DbWithJar<Jar> {}
// ANCHOR_END: jar_db_impl

/// Defines the setting that can be used to **configure** the compiler (e.g. the
/// **optimization level**, **target triple**, etc.) from the command line (e.g.
/// `leafc --opt-level=3`).
pub mod cli;
/// Defines the **settings** used to **tune** the compiler's behavior (e.g. the
/// **log level**, **log topic**, **optimization level**, etc.). These are
/// **not** set from the command line, but rather are set programmatically (e.g.
/// `leafc_log::set_log_level(LogLevel::Debug)`).
pub mod settings;

pub mod defs;

/// Defines the various **language-specific** implementations for various types
/// (e.g. `VariableId`, `FunctionId`, etc.) as well as generic tokens (e.g.
/// `FN_KW` => `fn`, `función`, etc.).
pub mod lang;
