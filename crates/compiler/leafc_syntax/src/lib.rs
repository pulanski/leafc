#![allow(unused_imports)] // TODO: Remove this once all nodes are implemented

pub mod ast;
mod syntax_gen;
mod syntax_kind_ext;
mod syntax_tree;

use std::{
    marker::PhantomData,
    sync::Arc,
};

pub use ast::generated::kinds::SyntaxKind;
use ast::AstNode;
use leafc_errors::SyntaxError;
use rowan::GreenNode;
pub use syntax_tree::{
    PreorderWithTokens,
    SyntaxElement,
    SyntaxElementChildren,
    SyntaxNode,
    SyntaxNodeChildren,
    SyntaxToken,
};
