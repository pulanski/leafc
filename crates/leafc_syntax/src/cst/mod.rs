#![allow(unused)]
//! This module contains the **typed AST** of Leaf. The **typed AST** is a
//! layer on top of the **untyped AST** (the `SyntaxNode` type). The typed
//! AST is a tree of `AstNode`s, which are thin wrappers around `SyntaxNode`s.
//! The conversion from `SyntaxNode` to `AstNode` is zero-cost, and the
//! `AstNode` type is used to provide a **strongly-typed** API for tree
//! traversal and manipulation.

pub mod generated;
mod node_ext;
pub mod node_id;
mod token_ext;
pub mod traits;

pub use generated::nodes::*;
use generated::tokens::*;
use traits::{
    HasAttrs,
    HasDocComments,
    HasModuleItem,
    HasName,
    HasVisibility,
};

use std::marker::PhantomData;

use itertools::Either;

use self::generated::kinds::SyntaxKind;
use crate::{
    syntax_tree::{
        SyntaxNode,
        SyntaxToken,
    },
    SyntaxNodeChildren,
};

/// ### A **typed** AST node.
///
/// Defines the behavior for converting between an **untyped** `SyntaxNode` and
/// a **typed** `AstNode`.
///
/// The conversion itself has no runtime cost since both `AstNode`s and
/// `SyntaxNode`s have exactly the same representation. That is, they both
/// contain a **pointer** to the **tree root** and a **pointer** to the **node
/// itself**.
///
/// The `AstNode` trait is implemented for all the AST nodes and enforces
/// the invariant that the specific `SyntaxNode` has the specific `SyntaxKind`.
/// // TODO: check on this: (e.g. `FnDef` can only be casted from a `FN_DEF`
/// node).
pub trait AstNode {
    /// Returns `true` if the syntax node has the **correct kind** for this AST
    /// node and as such can be converted to an `AstNode`. Otherwise,
    /// returns `false`.
    ///
    /// This is a **const fn**.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // TODO: update this example
    /// use leafc_syntax::{ast::FnDef, SyntaxKind};
    ///
    /// let fn_def = SyntaxNode::new_root(SyntaxKind::FN_DEF);
    /// assert!(FnDef::can_cast(fn_def.kind()));
    /// ```
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized;

    /// Casts the given syntax node to an `AstNode`, converting the **untyped**
    /// `SyntaxNode` to a **typed** `AstNode`, if the syntax node has the
    /// **correct kind**. Otherwise, returns `None`.
    ///
    /// This is a **const fn**.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // TODO: update this example
    /// ```
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized;

    /// Returns the underlying `SyntaxNode` that this `AstNode` wraps. This is
    /// the symmetric or inverse operation of `AstNode::cast`.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // TODO: update this example
    /// ```
    fn syntax(&self) -> &SyntaxNode;

    fn clone_for_update(&self) -> Self
    where
        Self: Sized,
    {
        Self::cast(self.syntax().clone_for_update()).unwrap()
    }
    fn clone_subtree(&self) -> Self
    where
        Self: Sized,
    {
        Self::cast(self.syntax().clone_subtree()).unwrap()
    }
}

/// ### A **typed** AST token.
///
/// Defines the behavior for converting between an **untyped** `SyntaxToken` and
/// a **typed** `AstToken`. The conversion itself has no runtime cost since both
/// `AstToken`s and `SyntaxToken`s have exactly the same representation. That
/// is, they both contain a **pointer** to the **token itself**.
///
/// The `AstToken` trait is implemented for all the AST tokens and enforces
/// the invariant that the specific `SyntaxToken` has the specific `SyntaxKind`.
///
/// **NOTE**: The current pipeline gathers lots of token metadata via `Logos`
/// and stores it in the database. TODO: want to have interned strings for
/// tokens, and make the lexing incremental or _"lazy"_ (i.e. if the same
/// character stream is lexed twice, the lexer should not re-lex it).
pub trait AstToken {
    /// Returns `true` if the syntax token has the **correct kind** for this AST
    /// token and as such can be converted to an `AstToken`. Otherwise,
    /// returns `false`.
    ///
    /// This is a **const fn**.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // TODO: update this example
    /// ```
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized;

    /// Casts the given syntax token to an `AstToken`, converting the
    /// **untyped** `SyntaxToken` to a **typed** `AstToken`, if the syntax
    /// token has the **correct kind**. Otherwise, returns `None`.
    ///
    /// This is a **const fn**.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // TODO: update this example
    /// ```
    fn cast(syntax: SyntaxToken) -> Option<Self>
    where
        Self: Sized;

    /// Returns the underlying `SyntaxToken` that this `AstToken` wraps. This is
    /// the symmetric or inverse operation of `AstToken::cast`.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // TODO: update this example
    /// ```
    fn syntax(&self) -> &SyntaxToken;

    fn text(&self) -> &str {
        self.syntax().text()
    }
}

/// An iterator over `SyntaxNode` children of a particular AST type.
#[derive(Debug, Clone)]
pub struct AstChildren<N> {
    inner: SyntaxNodeChildren,
    ph:    PhantomData<N>,
}

impl<N> AstChildren<N> {
    fn new(parent: &SyntaxNode) -> Self {
        AstChildren { inner: parent.children(), ph: PhantomData }
    }
}

impl<N: AstNode> Iterator for AstChildren<N> {
    type Item = N;
    fn next(&mut self) -> Option<N> {
        self.inner.find_map(N::cast)
    }
}

impl<L, R> AstNode for Either<L, R>
where
    L: AstNode,
    R: AstNode,
{
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        L::can_cast(kind) || R::can_cast(kind)
    }

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if L::can_cast(syntax.kind()) {
            L::cast(syntax).map(Either::Left)
        } else {
            R::cast(syntax).map(Either::Right)
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        self.as_ref().either(L::syntax, R::syntax)
    }
}

mod support {
    use super::{
        AstChildren,
        AstNode,
        SyntaxKind,
        SyntaxNode,
        SyntaxToken,
    };

    pub(super) fn child<N: AstNode>(parent: &SyntaxNode) -> Option<N> {
        parent.children().find_map(N::cast)
    }

    pub(super) fn children<N: AstNode>(parent: &SyntaxNode) -> AstChildren<N> {
        AstChildren::new(parent)
    }

    pub(super) fn token(parent: &SyntaxNode, kind: SyntaxKind) -> Option<SyntaxToken> {
        parent.children_with_tokens().filter_map(|it| it.into_token()).find(|it| it.kind() == kind)
    }
}

// pub struct Pat {
//     pub id: NodeId,
//     pub kind: PatKind,
// }

// pub struct Ty {
//     pub id: NodeId,
//     pub kind: TyKind,
//     pub span: Span,
//     pub tokens: TokenStream,
// }

// pub enum TyKind {
//     Tup(Vec<Ty>),

//     Slice(Box<Ty>),

//     Array(Box<Ty>, u64),
//     // Ptr(Box<Ty>),

//     // Ref(Box<Ty>),
//     // Path(Path),
// }
