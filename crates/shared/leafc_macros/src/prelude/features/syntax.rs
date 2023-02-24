// configure which syntax tree will be used, `rowan` or `cstree`

/// Defines the syntax tree being brought into scope (either [`rowan`] or
/// [`cstree`]) [default: [`rowan`]].
///
/// This feature is used to configure which syntax tree will be used, [`rowan`]
/// or [`cstree`], by the parser for constructing lossless syntax trees which
/// possess the property of structural sharing.
///
/// [`rowan`]: https://crates.io/crates/rowan
/// [`cstree`]: https://crates.io/crates/cstree
#[macro_export]
macro_rules! SYNTAX_TREE_FEATURE_USE_DECLS {
    () => {
        #[cfg(not(feature = "cstree"))]
        use rowan::{
            GreenNodeBuilder,
            Language,
        };

        #[cfg(feature = "cstree")]
        use cstree::{
            GreenNodeBuilder,
            Language,
        };
    };
}
