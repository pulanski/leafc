//! Defines database & queries for name resolution.

// TODO: API similar to below:

// #[salsa::query_group(InternDatabaseStorage)]
// pub trait InternDatabase: SourceDatabase {
//     #[salsa::interned]
//     fn intern_function(&self, loc: FunctionLoc) -> FunctionId;
//     #[salsa::interned]
//     fn intern_struct(&self, loc: StructLoc) -> StructId;
//     #[salsa::interned]
//     fn intern_union(&self, loc: UnionLoc) -> UnionId;
//     #[salsa::interned]
//     fn intern_enum(&self, loc: EnumLoc) -> EnumId;
//     #[salsa::interned]
//     fn intern_const(&self, loc: ConstLoc) -> ConstId;
//     #[salsa::interned]
//     fn intern_static(&self, loc: StaticLoc) -> StaticId;
//     #[salsa::interned]
//     fn intern_trait(&self, loc: TraitLoc) -> TraitId;
//     #[salsa::interned]
//     fn intern_type_alias(&self, loc: TypeAliasLoc) -> TypeAliasId;
//     #[salsa::interned]
//     fn intern_impl(&self, loc: ImplLoc) -> ImplId;
//     #[salsa::interned]
//     fn intern_extern_block(&self, loc: ExternBlockLoc) -> ExternBlockId;
//     #[salsa::interned]
//     fn intern_block(&self, loc: BlockLoc) -> BlockId;
//     #[salsa::interned]
//     fn intern_macro2(&self, loc: Macro2Loc) -> Macro2Id;
//     #[salsa::interned]
//     fn intern_proc_macro(&self, loc: ProcMacroLoc) -> ProcMacroId;
//     #[salsa::interned]
//     fn intern_macro_rules(&self, loc: MacroRulesLoc) -> MacroRulesId;
// }
