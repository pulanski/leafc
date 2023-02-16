use strum_macros::EnumIter;

pub type EmitKinds = Vec<EmitKind>;

/// The **kind of output** to emit from the compiler (e.g. the `AST`, `LLVM IR`,
/// etc.).
#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash, EnumIter)]
pub enum EmitKind {
    /// Emit the corresponding **token stream** for the input file.
    TokenStream,

    /// Emit the corresponding **`AST`** for the input file.
    Ast,

    /// Emit the corresponding **`LLVM IR`** for the input file.
    LlvmIr,

    /// Emit the corresponding **`object file`** for the input file.
    ObjectFile,

    /// Emit the corresponding **`LLVM bitcode`** for the input file.
    Bitcode,

    /// Emit the corresponding **`assembly code`** for the input file.
    Asm,
}
