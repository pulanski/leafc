pub use salsa;

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

/// Defines the **settings** used to **tune** the compiler's behavior (e.g. the **log level**, **log topic**,
/// **optimization level**, etc.).
pub mod settings;
