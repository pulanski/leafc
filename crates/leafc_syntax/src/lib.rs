#![allow(unused_imports)] // TODO: Remove this once all nodes are implemented
pub mod ast;
pub mod syntax_gen;
pub mod syntax_tree;

use std::{marker::PhantomData, sync::Arc};

pub use ast::generated::kinds::SyntaxKind;
use ast::AstNode;
use rowan::GreenNode;
pub use syntax_tree::{
    PreorderWithTokens, SyntaxElement, SyntaxElementChildren, SyntaxNode, SyntaxNodeChildren,
    SyntaxToken,
};

/// `Parse` is the result of the parsing: a syntax tree and a collection of
/// errors.
///
/// Note that we always produce a syntax tree, even for completely invalid
/// files.
#[derive(Debug, PartialEq, Eq)]
pub struct Parse<T> {
    green: GreenNode,
    // errors: Arc<Vec<SyntaxError>>,
    _ty: PhantomData<fn() -> T>,
}

// impl<T> Clone for Parse<T> {
//     fn clone(&self) -> Parse<T> {
//         Parse { green: self.green.clone(), errors: self.errors.clone(), _ty: PhantomData }
//     }
// }

// impl<T> Parse<T> {
//     fn new(green: GreenNode, errors: Vec<SyntaxError>) -> Parse<T> {
//         Parse { green, errors: Arc::new(errors), _ty: PhantomData }
//     }

//     pub fn syntax_node(&self) -> SyntaxNode {
//         SyntaxNode::new_root(self.green.clone())
//     }
//     pub fn errors(&self) -> &[SyntaxError] {
//         &self.errors
//     }
// }

// impl<T: AstNode> Parse<T> {
//     pub fn to_syntax(self) -> Parse<SyntaxNode> {
//         Parse { green: self.green, errors: self.errors, _ty: PhantomData }
//     }

//     pub fn tree(&self) -> T {
//         T::cast(self.syntax_node()).unwrap()
//     }

//     pub fn ok(self) -> Result<T, Arc<Vec<SyntaxError>>> {
//         if self.errors.is_empty() {
//             Ok(self.tree())
//         } else {
//             Err(self.errors)
//         }
//     }
// }
