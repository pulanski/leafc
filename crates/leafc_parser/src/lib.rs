use std::{
    marker::PhantomData,
    sync::Arc,
};

use rowan::GreenNode;

use leafc_errors::SyntaxError;
use leafc_syntax::{
    cst::{
        AstNode,
        SourceFile,
    },
    SyntaxNode,
};

/// `Parse` is the result of the parsing: a syntax tree and a collection of
/// errors.
///
/// Note that we always produce a syntax tree, even for completely invalid
/// files.
#[derive(Debug, PartialEq, Eq)]
pub struct Parse<T> {
    green:  GreenNode,
    errors: Arc<Vec<SyntaxError>>,
    _ty:    PhantomData<fn() -> T>,
}

impl<T> Clone for Parse<T> {
    fn clone(&self) -> Parse<T> {
        Parse { green: self.green.clone(), errors: self.errors.clone(), _ty: PhantomData }
    }
}

impl<T> Parse<T> {
    fn new(green: GreenNode, errors: Vec<SyntaxError>) -> Parse<T> {
        Parse { green, errors: Arc::new(errors), _ty: PhantomData }
    }

    pub fn syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }
    pub fn errors(&self) -> &[SyntaxError] {
        &self.errors
    }
}

impl<T: AstNode> Parse<T> {
    pub fn to_syntax(self) -> Parse<SyntaxNode> {
        Parse { green: self.green, errors: self.errors, _ty: PhantomData }
    }

    pub fn tree(&self) -> T {
        T::cast(self.syntax_node()).unwrap()
    }

    pub fn ok(self) -> Result<T, Arc<Vec<SyntaxError>>> {
        if self.errors.is_empty() {
            Ok(self.tree())
        } else {
            Err(self.errors)
        }
    }
}

// impl Parse<SourceFile> {
//     pub fn parse(source_text: &str) -> Parse<SourceFile> {
//         let (green, errors) = leafc_syntax::parse(source_text);
//         Parse::new(green, errors)
//     }
// }

#[cfg(test)]
mod api_walkthrough {
    // use crate::ast::SourceFile;

    use leafc_syntax::cst::SourceFile;

    #[test]
    fn parse_source_file() {
        let source_text = "
        fn foo_bar() {
            3 + 2
        }
";

        // let parse = leafc_parser::parse(source_text); // a generic parser for
        // any syntax tree

        // let parse = SourceFile::parse(source_text); // a parser for a
        // specific syntax tree

        // let parse = leafc_parser::parse::<ast::SourceFile>(source_text); // a
        // parser for a specific syntax tree
    }
}
