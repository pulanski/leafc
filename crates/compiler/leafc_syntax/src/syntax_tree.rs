#![allow(dead_code, unused)] // TODO: remove this
use {
    cfg_if::cfg_if,
    num_traits::{
        FromPrimitive,
        ToPrimitive,
    },
};

use {
    leafc_diagnostics::SyntaxError,
    leafc_macros::SYNTAX_TREE_FEATURE_USE_DECLS,
};

use leafc_lexer::token::Token;

use crate::ast::SyntaxKind;

SYNTAX_TREE_FEATURE_USE_DECLS!();

/// The Leaf language's **syntax tree**.
///
/// This is a tree that has been parsed from a source file or general source
/// text. It is **lossless** in the sense that it preserves all the information
/// from the source text (e.g. whitespace, comments etc.).
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum LeafLanguage {}

// Define various wrapper types around the underlying syntax tree
// implementation.
cfg_if! {
    // ------------------------------------------------------------------------
    // cstree-based syntax tree (experimental)
    // ------------------------------------------------------------------------
    if #[cfg(feature = "cstree")] {
        impl cstree::Language for LeafLanguage {
            type Kind = SyntaxKind;

            /// Converts a generic [`cstree`]-`SyntaxKind` into a [`SyntaxKind`][crate::syntax_tree::SyntaxKind]
            /// specific to the **Leaf language**.
            ///
            /// # Panics
            ///
            /// Panics if the `SyntaxKind` is not a valid `SyntaxKind` for the Leaf
            /// language.
            ///
            /// [`cstree`]: https://docs.rs/cstree
            fn kind_from_raw(raw: cstree::SyntaxKind) -> SyntaxKind {
                Self::Kind::from_u16(raw.0).unwrap()
                // SyntaxKind::from(raw.0)
            }

            /// Converts a [`SyntaxKind`][crate::syntax_tree::SyntaxKind] specific
            /// to the Leaf language into a generic [`cstree`]-`SyntaxKind`.
            ///
            /// # Panics
            ///
            /// Panics if the [`SyntaxKind`][crate::syntax_tree::SyntaxKind] is not
            /// a valid [`SyntaxKind`][crate::syntax_tree::SyntaxKind] for the Leaf
            /// language.
            ///
            /// [`cstree`]: https://docs.rs/cstree
            fn kind_to_raw(kind: SyntaxKind) -> rowan::SyntaxKind {
                rowan::SyntaxKind(kind.to_u16().unwrap())
                // rowan::SyntaxKind(kind.into())
                // TODO: add error handling here via miette
            }

        }

        /// [`leafc`]'s [`SyntaxNode`][crate::syntax_tree::SyntaxNode] type, a wrapper around
        /// [`cstree`]'s language-agnostic [`SyntaxNode`] type.
        ///
        /// These are analogous to a `RedNode` in the [**Red-Green**](https://ericlippert.com/2012/06/08/red-green-trees/) tree model seen in **Roslyn**.
        /// They signify non-terminal nodes in the syntax tree.
        ///
        /// **NOTE**: This is an **experimental** feature and is not guaranteed to
        /// be stable. If you want to use the default syntax tree implementation, you can **disable** the
        /// [`cstree`] feature flag.
        ///
        /// [`rowan`]: https://crates.io/crates/rowan
        /// [`cstree`]: https://crates.io/crates/cstree
        /// [`leafc`]: https://crates.io/crates/leafc
        /// [`SyntaxNode`]: crate::syntax_tree::SyntaxNode
        // #[doc = DEFAULT_SYNTAX_TREE_NOTE!("SyntaxNode")] // TODO
        pub type SyntaxNode = rowan::SyntaxNode<LeafLanguage>;

        /// [`leafc`]'s [`SyntaxToken`][crate::syntax_tree::SyntaxToken] type, a wrapper around
        /// [`cstree`]'s language-agnostic [`SyntaxToken`] type.
        ///
        /// These are analogous to a `RedNode` in the [**Red-Green**](https://ericlippert.com/2012/06/08/red-green-trees/) tree model seen in **Roslyn**, however, they signify terminal nodes in the syntax
        /// tree aka **tokens**.
        ///
        /// **NOTE**: This is an **experimental** feature and is not guaranteed to
        /// be stable. If you want to use the default syntax tree implementation, you can **disable** the
        /// [`cstree`] feature flag.
        ///
        /// [`rowan`]: https://crates.io/crates/rowan
        /// [`cstree`]: https://crates.io/crates/cstree
        /// [`leafc`]: https://crates.io/crates/leafc
        /// [`SyntaxToken`]: https://docs.rs/rowan/0.15.10/rowan/api/struct.SyntaxToken.html
        pub type SyntaxToken = rowan::SyntaxToken<LeafLanguage>;

        /// [`leafc`]'s [`SyntaxElement`][crate::syntax_tree::SyntaxElement] type, a wrapper around
        /// [`cstree`]'s language-agnostic [`SyntaxElement`] type.
        ///
        /// These are analogous to a `GreenNode` in the [**Red-Green**](https://ericlippert.com/2012/06/08/red-green-trees/) tree model seen in **Roslyn**.
        ///
        /// **NOTE**: This is an **experimental** feature and is not guaranteed to
        /// be stable. If you want to use the default syntax tree implementation, you can **disable** the
        /// [`cstree`] feature flag.
        ///
        /// [`rowan`]: https://crates.io/crates/rowan
        /// [`cstree`]: https://crates.io/crates/cstree
        /// [`leafc`]: https://crates.io/crates/leafc
        /// [`SyntaxElement`]: https://docs.rs/rowan/0.15.10/rowan/api/struct.SyntaxElement.html
        pub type SyntaxElement = rowan::SyntaxElement<LeafLanguage>;

        /// Leafc's [`SyntaxNodeChildren`][crate::syntax_tree::SyntaxNodeChildren] type, a wrapper around
        /// [`cstree`]'s language-agnostic [`SyntaxNodeChildren`] type.
        ///
        /// **NOTE**: This is an **experimental** feature and is not guaranteed to
        /// be stable. If you want to use the default syntax tree implementation, you can **disable** the
        /// [`cstree`] feature flag.
        ///
        /// [`rowan`]: https://crates.io/crates/rowan
        /// [`cstree`]: https://crates.io/crates/cstree
        /// [`leafc`]: https://crates.io/crates/leafc
        /// [`SyntaxNodeChildren`]: https://docs.rs/rowan/0.15.10/rowan/api/struct.SyntaxNodeChildren.html
        pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<LeafLanguage>;

        /// [`leafc`]'s [`SyntaxElementChildren`][crate::syntax_tree::SyntaxElementChildren] type, a wrapper
        /// around [`cstree`]'s language-agnostic [`SyntaxElementChildren`] type.
        ///
        /// **NOTE**: This is an **experimental** feature and is not guaranteed to
        /// be stable. If you want to use the default syntax tree implementation, you can **disable** the
        /// [`cstree`] feature flag.
        ///
        /// [`rowan`]: https://crates.io/crates/rowan
        /// [`cstree`]: https://crates.io/crates/cstree
        /// [`leafc`]: https://crates.io/crates/leafc
        /// [`SyntaxNodeChildren`]: https://docs.rs/rowan/0.15.10/rowan/api/struct.SyntaxNodeChildren.html
        pub type SyntaxElementChildren = rowan::SyntaxElementChildren<LeafLanguage>;

        /// [`leafc`]'s [`PreorderWithTokens`][crate::syntax_tree::PreorderWithTokens]
        /// type, a wrapper around [`rowan`]'s language-agnostic [`PreorderWithTokens`] type.
        ///
        /// **NOTE**: This is an **experimental** feature and is not guaranteed to
        /// be stable. If you want to use the default syntax tree implementation, you can **disable** the
        /// [`cstree`] feature flag.
        ///
        /// [`rowan`]: https://crates.io/crates/rowan
        /// [`cstree`]: https://crates.io/crates/cstree
        /// [`leafc`]: https://crates.io/crates/leafc
        /// [`PreorderWithTokens`]: https://docs.rs/rowan/0.15.10/rowan/api/struct.PreorderWithTokens.html
        pub type PreorderWithTokens = rowan::api::PreorderWithTokens<LeafLanguage>;

    // ------------------------------------------------------------------------
    // Rowan-based syntax tree
    // ------------------------------------------------------------------------
    } else {
        impl rowan::Language for LeafLanguage {
            type Kind = SyntaxKind;

            /// Converts a generic rowan-`SyntaxKind` into a [`SyntaxKind`][crate::syntax_tree::SyntaxKind]
            /// specific to the Leaf language.
            ///
            /// # Panics
            ///
            /// Panics if the `SyntaxKind` is not a valid `SyntaxKind` for the Leaf
            /// language.
            fn kind_from_raw(raw: rowan::SyntaxKind) -> SyntaxKind {
                Self::Kind::from_u16(raw.0).unwrap()
                // SyntaxKind::from(raw.0)
            }

            /// Converts a `SyntaxKind` specific to the Leaf language into a generic
            /// rowan-`SyntaxKind`.
            ///
            /// # Panics
            ///
            /// Panics if the `SyntaxKind` is not a valid `SyntaxKind` for the Leaf
            /// language.
            fn kind_to_raw(kind: SyntaxKind) -> rowan::SyntaxKind {
                rowan::SyntaxKind(kind.to_u16().unwrap())
                // rowan::SyntaxKind(kind.into())
                // TODO: add error handling here via miette
            }
        }

        /// [`leafc`]'s [`SyntaxNode`][crate::syntax_tree::SyntaxNode] type, a wrapper around
        /// [`rowan`]'s language-agnostic [`SyntaxNode`] type.
        ///
        /// These are analogous to a `RedNode` in the [**Red-Green**](https://ericlippert.com/2012/06/08/red-green-trees/) tree model seen in **Roslyn**.
        /// They signify non-terminal nodes in the syntax tree.
        ///
        /// **NOTE**: This is the default type for [`SyntaxNode`] in Leafc and uses [`rowan`]
        /// as the underlying syntax tree implementation. If you want to use a different syntax tree
        /// implementation, you can use the [`cstree`] feature flag. It is important to note that
        /// the [`cstree`] feature flag is **experimental** and is not guaranteed to be stable.
        ///
        /// [`rowan`]: https://crates.io/crates/rowan
        /// [`cstree`]: https://crates.io/crates/cstree
        /// [`leafc`]: https://crates.io/crates/leafc
        /// [`SyntaxNode`]: crate::syntax_tree::SyntaxNode
        // #[doc = DEFAULT_SYNTAX_TREE_NOTE!("SyntaxNode")] // TODO
        pub type SyntaxNode = rowan::SyntaxNode<LeafLanguage>;

        /// [`leafc`]'s [`SyntaxToken`][crate::syntax_tree::SyntaxToken] type, a wrapper around
        /// [`rowan`]'s language-agnostic [`SyntaxToken`] type.
        ///
        /// These are analogous to a `RedNode` in the [**Red-Green**](https://ericlippert.com/2012/06/08/red-green-trees/) tree model seen in **Roslyn**, however, they signify terminal nodes in the syntax tree aka **tokens**.
        ///
        /// **NOTE**: This is the default type for [`SyntaxToken`] in Leafc and uses [`rowan`]
        /// as the underlying syntax tree implementation. If you want to use a different syntax tree
        /// implementation, you can use the [`cstree`] feature flag. It is important to note that
        /// the [`cstree`] feature flag is **experimental** and is not guaranteed to be stable.
        ///
        /// [`rowan`]: https://crates.io/crates/rowan
        /// [`cstree`]: https://crates.io/crates/cstree
        /// [`leafc`]: https://crates.io/crates/leafc
        /// [`SyntaxToken`]: https://docs.rs/rowan/0.15.10/rowan/api/struct.SyntaxToken.html
        pub type SyntaxToken = rowan::SyntaxToken<LeafLanguage>;

        /// [`leafc`]'s [`SyntaxElement`][crate::syntax_tree::SyntaxElement] type, a wrapper around
        /// [`rowan`]'s language-agnostic [`SyntaxElement`] type.
        ///
        /// These are analogous to a `GreenNode` in the [**Red-Green**](https://ericlippert.com/2012/06/08/red-green-trees/) tree model seen in **Roslyn**.
        ///
        /// **NOTE**: This is the default type for [`SyntaxElement`] in Leafc and uses [`rowan`]
        /// as the underlying syntax tree implementation. If you want to use a different syntax tree
        /// implementation, you can use the [`cstree`] feature flag. It is important to note that
        /// the [`cstree`] feature flag is **experimental** and is not guaranteed to be stable.
        ///
        /// [`rowan`]: https://crates.io/crates/rowan
        /// [`cstree`]: https://crates.io/crates/cstree
        /// [`leafc`]: https://crates.io/crates/leafc
        /// [`SyntaxElement`]: https://docs.rs/rowan/0.15.10/rowan/api/struct.SyntaxElement.html
        pub type SyntaxElement = rowan::SyntaxElement<LeafLanguage>;

        /// Leafc's [`SyntaxNodeChildren`][crate::syntax_tree::SyntaxNodeChildren] type, a wrapper around
        /// [`rowan`]'s language-agnostic [`SyntaxNodeChildren`] type.
        ///
        /// **NOTE**: This is the default type for [`SyntaxNodeChildren`] in Leafc and uses [`rowan`]
        /// as the underlying syntax tree implementation. If you want to use a different syntax tree
        /// implementation, you can use the [`cstree`] feature flag. It is important to note that
        /// the [`cstree`] feature flag is **experimental** and is not guaranteed to be stable.
        ///
        /// [`rowan`]: https://crates.io/crates/rowan
        /// [`cstree`]: https://crates.io/crates/cstree
        /// [`leafc`]: https://crates.io/crates/leafc
        /// [`SyntaxNodeChildren`]: https://docs.rs/rowan/0.15.10/rowan/api/struct.SyntaxNodeChildren.html
        pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<LeafLanguage>;

        /// [`leafc`]'s [`SyntaxElementChildren`][crate::syntax_tree::SyntaxElementChildren] type, a wrapper
        /// around [`rowan`]'s language-agnostic [`SyntaxElementChildren`] type.
        ///
        /// **NOTE**: This is the default type for [`SyntaxElementChildren`] in Leafc and uses [`rowan`]
        /// as the underlying syntax tree implementation. If you want to use a different syntax tree
        /// implementation, you can use the [`cstree`] feature flag. It is important to note that
        /// the [`cstree`] feature flag is **experimental** and is not guaranteed to be stable.
        ///
        /// [`rowan`]: https://crates.io/crates/rowan
        /// [`cstree`]: https://crates.io/crates/cstree
        /// [`leafc`]: https://crates.io/crates/leafc
        /// [`SyntaxNodeChildren`]: https://docs.rs/rowan/0.15.10/rowan/api/struct.SyntaxNodeChildren.html
        pub type SyntaxElementChildren = rowan::SyntaxElementChildren<LeafLanguage>;

        /// [`leafc`]'s [`PreorderWithTokens`][crate::syntax_tree::PreorderWithTokens]
        /// type, a wrapper around [`rowan`]'s language-agnostic [`PreorderWithTokens`] type.
        ///
        /// **NOTE**: This is the default type for [`PreorderWithTokens`] in Leafc and uses [`rowan`]
        /// as the underlying syntax tree implementation. If you want to use a different syntax tree
        /// implementation, you can use the [`cstree`] feature flag. It is important to note that
        /// the [`cstree`] feature flag is **experimental** and is not guaranteed to be stable.
        ///
        /// [`rowan`]: https://crates.io/crates/rowan
        /// [`cstree`]: https://crates.io/crates/cstree
        /// [`leafc`]: https://crates.io/crates/leafc
        /// [`PreorderWithTokens`]: https://docs.rs/rowan/0.15.10/rowan/api/struct.PreorderWithTokens.html
        pub type PreorderWithTokens = rowan::api::PreorderWithTokens<LeafLanguage>;
    }
}

/// A **builder** for a **concrete syntax tree**.
/// This is a tree that has been parsed from a source file or general source
/// text. It is **lossless** in the sense that it preserves all the information
/// from the source text (_e.g. whitespace, comments etc._).
///
/// This builder is used to construct a concrete syntax tree from a stream of
/// tokens. The builder is a **push**-based API, where the caller pushes tokens
/// into the builder and the builder constructs the tree.
///
/// # Example
///
/// ```rust,ignore
/// use leaf_lang::syntax::{SyntaxKind, SyntaxTreeBuilder};
///
/// let mut builder = SyntaxTreeBuilder::default();
///
/// builder.start_node(SyntaxKind::ROOT);
/// builder.token(SyntaxKind::INT, "1");
/// builder.finish_node();
///
/// let tree = builder.finish();
///
/// assert_eq!(tree.root().kind(), SyntaxKind::ROOT);
/// ```
#[derive(Debug, Default)]
pub struct SyntaxTreeBuilder {
    builder: GreenNodeBuilder<'static>,
}

impl SyntaxTreeBuilder {
    /// Creates a new **syntax tree builder**.
    /// This is useful for creating a syntax tree from a given syntax kind
    /// during parsing.
    pub fn new() -> Self {
        Self { builder: GreenNodeBuilder::new() }
    }

    /// Creates a _new_ **syntax tree** with the given **root** node.
    /// This is useful for creating a syntax tree from a given syntax kind
    /// during parsing.
    ///
    /// # Arguments
    ///
    /// * `root` - The root node of the syntax tree.
    ///
    /// # Returns
    ///
    /// A new syntax tree with the given root node.
    #[inline]
    pub fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(LeafLanguage::kind_to_raw(kind))
    }

    /// Adds a **token** to the current node via its **raw components** (_kind_
    /// and _text_).
    ///
    /// # Arguments
    ///
    /// * `kind` - The kind of the token.
    /// * `text` - The text of the token.
    #[inline]
    pub fn add_raw_token(&mut self, kind: SyntaxKind, text: &str) {
        self.builder.token(LeafLanguage::kind_to_raw(kind), text);
    }

    /// Adds a **token** created from the [`Lexer`] to the current node.
    /// This is useful for adding tokens to the syntax tree during parsing.
    ///
    /// # Arguments
    ///
    /// * `token` - The token to add to the syntax tree.
    #[inline]
    pub fn add_token(&mut self, token: &Token) {
        self.builder
            .token(LeafLanguage::kind_to_raw(SyntaxKind::from(*token.kind())), token.lexeme())
    }

    /// Finishes the current node.
    /// This must be called after a call to `start_node`.
    #[inline]
    pub fn finish_node(&mut self) {
        self.builder.finish_node()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    cfg_if! {
        if #[cfg(feature = "cstree")] {
            #[test]
            fn test() {
                let mut builder = SyntaxTreeBuilder::default();

                builder.start_node(SyntaxKind::ROOT);
                builder.token(SyntaxKind::INT, "1");
                builder.finish_node();

                let tree = builder.finish();

                assert_eq!(tree.root().kind(), SyntaxKind::ROOT);
            }

        } else {
            // #[test]
            // fn test() {
            //     let mut builder = ConcreteSyntaxTreeBuilder::default();

            //     builder.start_node(SyntaxKind::ROOT);
            //     builder.token(SyntaxKind::INT, "1");
            //     builder.finish_node();

            //     let tree = builder.finish();

            //     assert_eq!(tree.root().kind(), SyntaxKind::ROOT);
            // }
        }
    }
}
