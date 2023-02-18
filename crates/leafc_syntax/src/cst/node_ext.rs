use crate::{
    cst,
    SyntaxKind,
    SyntaxNode,
};
use smol_str::SmolStr;

use super::{
    generated::nodes::{
        ExprStmt,
        Item,
        Stmt,
    },
    AstNode,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AttrKind {
    Inner,
    Outer,
}

impl AttrKind {
    /// Returns `true` if the attr_kind is [`Inner`](Self::Inner).
    pub fn is_inner(&self) -> bool {
        matches!(self, Self::Inner)
    }

    /// Returns `true` if the attr_kind is [`Outer`](Self::Outer).
    pub fn is_outer(&self) -> bool {
        matches!(self, Self::Outer)
    }
}

impl cst::Attr {
    pub fn as_simple_atom(&self) -> Option<SmolStr> {
        let meta = self.meta()?;
        // if meta.eq_token().is_some() || meta.token_tree().is_some() {
        if meta.eq_token().is_some() || meta.token_tree().is_some() {
            return None;
        }
        self.simple_name()
    }

    pub fn as_simple_call(&self) -> Option<(SmolStr, cst::TokenTree)> {
        let tt = self.meta()?.token_tree()?;
        Some((self.simple_name()?, tt))
    }

    pub fn simple_name(&self) -> Option<SmolStr> {
        let path = self.meta()?.path()?;
        match (path.segment(), path.qualifier()) {
            (Some(segment), None) => Some(segment.syntax().first_token()?.text().into()),
            _ => None,
        }
    }

    pub fn kind(&self) -> AttrKind {
        match self.excl_token() {
            Some(_) => AttrKind::Inner,
            None => AttrKind::Outer,
        }
    }

    pub fn path(&self) -> Option<cst::Path> {
        self.meta()?.path()
    }

    pub fn expr(&self) -> Option<cst::Expr> {
        self.meta()?.expr()
    }

    pub fn token_tree(&self) -> Option<cst::TokenTree> {
        self.meta()?.token_tree()
    }
}

impl cst::HasAttrs for cst::AnyHasDocComments {}

// Stmt is the only nested enum, so it's easier to just hand-write it
#[allow(non_snake_case, bindings_with_variant_name)]
impl AstNode for Stmt {
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            EXPR_STMT => true,
            // LET_STMT | EXPR_STMT => true,
            _ => false,
            // _ => Item::can_cast(kind),
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            // LET_STMT => Stmt::LetStmt(LetStmt { syntax }),
            EXPR_STMT => Stmt::ExprStmt(ExprStmt { syntax }),
            _ => {
                let item = Item::cast(syntax)?;
                Stmt::Item(item)
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            // Stmt::LetStmt(it) => &it.syntax,
            Stmt::ExprStmt(it) => &it.syntax,
            Stmt::Item(it) => it.syntax(),
        }
    }
}
