#![allow(mixed_script_confusables)]
//! The lexer for Leaf.
//!
//! This module contains the lexer for Leaf, which is used to tokenize
//! Leaf source code into a stream of tokens.

pub mod token;

pub mod lexer;

pub use token::TokenKind;

pub use lexer::{
    lossless_lex,
    lossy_lex,
};
