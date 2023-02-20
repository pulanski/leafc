use leafc_errors::SyntaxError;
use num_traits::{
    FromPrimitive,
    ToPrimitive,
};

use rowan::{
    GreenNodeBuilder,
    Language,
};

use crate::cst::generated::kinds::SyntaxKind;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum LeafLanguage {}

impl rowan::Language for LeafLanguage {
    type Kind = SyntaxKind;

    /// Converts a generic rowan-`SyntaxKind` into a `SyntaxKind` specific to
    /// the Leaf language.
    ///
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

/// Leaf's `SyntaxNode` type, a wrapper around **rowan**'s language-agnostic
/// `SyntaxNode`.
///
/// These are analogous to a `RedNode` in the [**Red-Green**](https://ericlippert.com/2012/06/08/red-green-trees/) tree model seen in **Roslyn**.
pub type SyntaxNode = rowan::SyntaxNode<LeafLanguage>;

/// Leaf's `SyntaxToken` type, a wrapper around **rowan**'s language-agnostic
/// `SyntaxToken`.
///
/// These are analogous to a `RedNode` in the [**Red-Green**](https://ericlippert.com/2012/06/08/red-green-trees/)
/// tree model seen in **Roslyn**, however, they signify terminal nodes in the
/// syntax tree aka **tokens**.
pub type SyntaxToken = rowan::SyntaxToken<LeafLanguage>;

/// Leaf's `SyntaxElement` type, a wrapper around **rowan**'s language-agnostic
/// `SyntaxElement`.
pub type SyntaxElement = rowan::SyntaxElement<LeafLanguage>;

/// Leaf's `SyntaxNodeChildren` type, a wrapper around **rowan**'s
/// language-agnostic `SyntaxNodeChildren`.
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<LeafLanguage>;

/// Leaf's `SyntaxElementChildren` type, a wrapper around **rowan**'s
/// language-agnostic `SyntaxElementChildren`.
pub type SyntaxElementChildren = rowan::SyntaxElementChildren<LeafLanguage>;

/// Leaf's `SyntaxNodeChildrenWithTokens` type, a wrapper around **rowan**'s
/// language-agnostic `SyntaxNodeChildrenWithTokens`.
pub type PreorderWithTokens = rowan::api::PreorderWithTokens<LeafLanguage>;

/// A **builder** for a **concrete syntax tree**.
/// This is a tree that has been parsed from a source file or general source
/// text. It is **lossless** in the sense that it preserves all the information
/// from the source text (e.g. whitespace, comments etc.).
///
/// This builder is used to construct a concrete syntax tree from a stream of
/// tokens. The builder is a **push**-based API, where the caller pushes tokens
/// into the builder and the builder constructs the tree.
///
/// # Example
///
/// ```rust,ignore
/// use leaf_lang::syntax::{SyntaxKind, ConcreteSyntaxTreeBuilder};
///
/// let mut builder = ConcreteSyntaxTreeBuilder::default();
///
/// builder.start_node(SyntaxKind::ROOT);
/// builder.token(SyntaxKind::INT, "1");
/// builder.finish_node();
///
/// let tree = builder.finish();
///
/// assert_eq!(tree.root().kind(), SyntaxKind::ROOT);
/// ```
pub struct ConcreteSyntaxTreeBuilder {
    builder: GreenNodeBuilder<'static>,
    errors:  Vec<SyntaxError>,
}

impl ConcreteSyntaxTreeBuilder {
    /// Creates a new tree with the given root node.
    pub fn start_node(&mut self, kind: SyntaxKind) {
        let kind = LeafLanguage::kind_to_raw(kind);

        self.builder.start_node(kind);
    }

    /// Adds a **token** to the tree.
    pub fn token(&mut self, kind: SyntaxKind, text: &str) {
        let kind = LeafLanguage::kind_to_raw(kind);

        self.builder.token(kind, text);
    }

    /// Finishes the current node.
    /// This must be called after a call to `start_node`.
    pub fn finish_node(&mut self) {
        self.builder.finish_node();
    }
}
