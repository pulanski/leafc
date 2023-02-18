#![allow(unused_imports)] // TODO: Remove this once all nodes are implemented
pub mod cst;
pub mod syntax_gen;
pub mod syntax_tree;

use std::{
    marker::PhantomData,
    sync::Arc,
};

pub use cst::generated::kinds::SyntaxKind;
use cst::AstNode;
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
