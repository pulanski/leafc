// TODO: create an generic experimental_note macro that prints a note about the
// feature being experimental

// macro_rules! EXPERIMENTAL_SYNTAX_TREE_NOTE {
//     ($feature:expr, $method:expr) => {
//         format!(
//             "NOTE: The syntax tree feature `{}` is experimental, and the
// method `{}` may change \              in the future.",
//             $feature, $method,
//         );
//     };
// }

macro_rules! DOCUMENTATION_REFERENCES {
    ($syntax_tree_feature:expr) => {
        format!(
            "[`{}`]: https://docs.rs/leafc_syntax/{version}/leafc_syntax/{syntax_tree_feature}/struct.{syntax_tree_feature}.html",
            $syntax_tree_feature,
            version = env!("CARGO_PKG_VERSION"),
            syntax_tree_feature = $syntax_tree_feature,
        );
    };
}

// TODO: implement this
macro_rules! DEFAULT_SYNTAX_TREE_NOTE {
    ($syntax_tree_feature:expr) => {
        // format!(
        //     "NOTE: This is the default type for [`SyntaxNode`] in Leafc and uses
        // [`rowan`] as the \      underlying syntax tree implementation. If you want to
        // use a different syntax tree \      implementation, you can use the
        // [`{syntax_tree_feature}`] feature flag. It is \      important to note that
        // the [`{syntax_tree_feature}`] feature flag is \      **experimental** and is
        // not guaranteed to be stable.",     syntax_tree_feature =
        // $syntax_tree_feature, )

        // **NOTE**: This is the default type for [`SyntaxNode`] in Leafc and uses
        // [`rowan`] /// as the underlying syntax tree implementation. If you want
        // to use a different syntax tree /// implementation, you can use the
        // [`cstree`] feature flag. It is important to note that /// the
        // [`cstree`] feature flag is **experimental** and is not guaranteed to
        // be stable.
    };
}

// example:
//
// EXPERIMENTAL_NOTE!(CSTree, "SyntaxNodeChildren");
// EXPERIMENTAL_NOTE!(CSTree, "SyntaxNode");
//
// pub enum ExperimentalFeature {
//     CSTree,
// }
