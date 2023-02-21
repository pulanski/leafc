#![allow(mixed_script_confusables)]
#![feature(proc_macro_hygiene)]

/// The **READ-ME** file for the lexer.
macro_rules! LEXER_README {
    () => {
        include_str!("../LEXER.md")
    };
}

#[cfg_attr(doc, aquamarine::aquamarine)]
#[doc = LEXER_README!()]
/// ```mermaid
/// graph LR
///     s([Source]) --> a[[aquamarine]]
///     r[[rustdoc]] --> f([Docs w/ Mermaid!])
///     subgraph rustc[Rust Compiler]
///     a -. inject mermaid.js .-> r
///     end
/// ```
pub mod lexer;

pub mod token;

pub use {
    lexer::{
        lossless_lex,
        lossy_lex,
    },
    token::TokenKind,
};
