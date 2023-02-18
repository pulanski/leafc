//! Various traits that are implemented by ast nodes.
//!
//! The implementations are usually trivial, and live in generated.rs
use itertools::Either;

use crate::{
    cst::{
        self,
        support,
        AstChildren,
        AstNode,
        AstToken,
    },
    syntax_tree::SyntaxElementChildren,
    SyntaxElement,
    SyntaxToken,
    T,
};

pub trait HasName: AstNode {
    fn name(&self) -> Option<cst::Name> {
        support::child(self.syntax())
    }
}

pub trait HasVisibility: AstNode {
    fn visibility(&self) -> Option<cst::Visibility> {
        support::child(self.syntax())
    }
}

// ---

// pub trait HasLoopBody: AstNode {
//     fn loop_body(&self) -> Option<ast::BlockExpr> {
//         support::child(self.syntax())
//     }

//     fn label(&self) -> Option<ast::Label> {
//         support::child(self.syntax())
//     }
// }

// pub trait HasArgList: AstNode {
//     fn arg_list(&self) -> Option<ast::ArgList> {
//         support::child(self.syntax())
//     }
// }

pub trait HasModuleItem: AstNode {
    fn items(&self) -> AstChildren<cst::Item> {
        support::children(self.syntax())
    }
}

// pub trait HasGenericParams: AstNode {
//     fn generic_param_list(&self) -> Option<ast::GenericParamList> {
//         support::child(self.syntax())
//     }

//     fn where_clause(&self) -> Option<ast::WhereClause> {
//         support::child(self.syntax())
//     }
// }

// pub trait HasTypeBounds: AstNode {
//     fn type_bound_list(&self) -> Option<ast::TypeBoundList> {
//         support::child(self.syntax())
//     }

//     fn colon_token(&self) -> Option<SyntaxToken> {
//         support::token(self.syntax(), T![:])
//     }
// }

// ----

pub trait HasAttrs: AstNode {
    fn attrs(&self) -> AstChildren<cst::Attr> {
        support::children(self.syntax())
    }
    fn has_atom_attr(&self, atom: &str) -> bool {
        self.attrs().filter_map(|x| x.as_simple_atom()).any(|x| x == atom)
    }
}

pub trait HasDocComments: HasAttrs {
    fn doc_comments(&self) -> DocCommentIter {
        DocCommentIter { iter: self.syntax().children_with_tokens() }
    }
    fn doc_comments_and_attrs(&self) -> AttrDocCommentIter {
        AttrDocCommentIter { iter: self.syntax().children_with_tokens() }
    }
}

impl DocCommentIter {
    pub fn from_syntax_node(syntax_node: &cst::SyntaxNode) -> DocCommentIter {
        DocCommentIter { iter: syntax_node.children_with_tokens() }
    }

    #[cfg(test)]
    pub fn doc_comment_text(self) -> Option<String> {
        let docs = itertools::Itertools::join(
            &mut self.filter_map(|comment| comment.doc_comment().map(ToOwned::to_owned)),
            "\n",
        );
        if docs.is_empty() {
            None
        } else {
            Some(docs)
        }
    }
}

pub struct DocCommentIter {
    iter: SyntaxElementChildren,
}

impl Iterator for DocCommentIter {
    type Item = cst::Comment;
    fn next(&mut self) -> Option<cst::Comment> {
        self.iter.by_ref().find_map(|el| {
            el.into_token().and_then(cst::Comment::cast).filter(cst::Comment::is_doc)
        })
    }
}

pub struct AttrDocCommentIter {
    iter: SyntaxElementChildren,
}

impl AttrDocCommentIter {
    pub fn from_syntax_node(syntax_node: &cst::SyntaxNode) -> AttrDocCommentIter {
        AttrDocCommentIter { iter: syntax_node.children_with_tokens() }
    }
}

impl Iterator for AttrDocCommentIter {
    type Item = Either<cst::Attr, cst::Comment>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.by_ref().find_map(|el| match el {
            SyntaxElement::Node(node) => cst::Attr::cast(node).map(Either::Left),
            SyntaxElement::Token(tok) => {
                cst::Comment::cast(tok).filter(cst::Comment::is_doc).map(Either::Right)
            }
        })
    }
}
