#![allow(dead_code)]
#![doc = include_str!("../../TOKEN.md")]

mod utils;

use logos::Logos;

#[allow(non_camel_case_types)]
#[derive(Debug, Logos, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TokenKind {
    #[error]
    // #[regex(r"[ \t\f]+", logos::skip)]
    ERROR,

    #[regex("//[^\r\n]*", priority = 2)]
    #[regex("//[^\n]*", priority = 1)]
    Comment,

    #[regex("///[^\r\n]*", priority = 4)]
    #[regex("///[^\n]*", priority = 3)]
    DocComment,

    #[regex(r"\s+")]
    WHITESPACE,

    #[regex("b?'[^']*'")]
    Rune,
    #[regex(r#"b?"(\\.|[^\\"])*""#)]
    String,

    #[token("inf")]
    #[token("NaN")]
    #[regex(r#"[+-]?[0-9][0-9_]*\.[0-9][0-9_]*([eE][+-]?[0-9][0-9_]*)?"#)]
    #[regex(
        r#"[+-]?0x[0-9a-fA-F][0-9a-fA-F_]*\.[0-9a-fA-F][0-9a-fA-F_]*([pP][+-]?[0-9][0-9_]?)?"#
    )]
    Float,

    #[regex("[+-]?[0-9][0-9_]*")]
    #[regex("[+-]?0b[0-1][0-1_]*")]
    #[regex("[+-]?0x[0-9a-fA-F][0-9a-fA-F_]*")]
    INTEGER,
    #[token("true")]
    #[token("false")]
    Bool,

    #[regex(r"[\p{XID_Start}\p{Emoji_Presentation}][\p{XID_Continue}\p{Emoji_Presentation}]*")]
    IDENT,

    #[token("fn")]
    FN_KW,
    #[token("import")]
    Import,
    #[token("let")]
    Let,
    #[token("type")]
    Type,
    #[token("enum")]
    Enum,
    #[token("trait")]
    Trait,
    #[token("const")]
    Const,
    #[token("extend")]
    Extend,
    #[token("with")]
    With,
    #[token("alias")]
    Alias,
    #[token("mut")]
    Mut,
    #[token("ref")]
    Ref,
    #[token("extern")]
    Extern,
    #[token("unsafe")]
    Unsafe,

    #[token("in")]
    In,
    #[token("is")]
    Is,
    #[token("loop")]
    Loop,
    #[token("while")]
    While,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("then")]
    Then,
    #[token("for")]
    For,
    #[token("return")]
    RETURN_KW,
    #[token("continue")]
    Continue,
    #[token("break")]
    Break,
    #[token("match")]
    Match,

    #[token("exposing")]
    Exposing,
    #[token("export")]
    Export,
    #[token("as")]
    As,
    #[token("lib")]
    Library,
    #[token("end")]
    End,
    #[token("pkg")]
    Package,
    #[token("exposed")]
    Exposed,
    #[token("empty")]
    Empty,

    #[token("or")]
    Or,
    #[token("and")]
    And,
    #[token("where")]
    Where,

    #[token("=")]
    Equal,
    #[token("+=")]
    AddAssign,
    #[token("-=")]
    SubAssign,
    #[token("*=")]
    MultAssign,
    #[token("/=")]
    DivAssign,
    #[token("%=")]
    ModAssign,
    #[token("**=")]
    PowAssign,
    #[token("<<=")]
    ShlAssign,
    #[token(">>=")]
    ShrAssign,
    #[token("|=")]
    OrAssign,
    #[token("&=")]
    AndAssign,
    #[token("^=")]
    XorAssign,

    #[token("==")]
    IsEqual,
    #[token("!=")]
    IsNotEqual,
    #[token(">=")]
    GreaterThanEqual,
    #[token("<=")]
    LessThanEqual,
    #[token("<")]
    LeftCaret,
    #[token(">")]
    RightCaret,

    #[token("!")]
    Bang,

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("/")]
    Divide,
    #[token("*")]
    Star,
    #[token("%")]
    Modulo,
    #[token("**")]
    DoubleStar,
    #[token("<<")]
    Shl,
    #[token(">>")]
    Shr,
    #[token("|")]
    Pipe,
    #[token("&")]
    Ampersand,
    #[token("^")]
    Caret,

    #[token("[")]
    LeftBrace,
    #[token("]")]
    RightBrace,
    #[token("(")]
    L_PAREN,
    #[token(")")]
    R_PAREN,
    #[token("{")]
    L_BRACE,
    #[token("}")]
    R_BRACE,
    #[token("->")]
    THIN_ARROW,
    #[token("<-")]
    LeftArrow,
    #[token("=>")]
    RightRocket,

    #[token("@")]
    AtSign,
    #[token(",")]
    Comma,
    #[token(";")]
    SEMICOLON,
    #[token(":")]
    Colon,
    #[token(".")]
    Dot,
    #[token("..")]
    DoubleDot,
}

impl TokenKind {
    pub fn is_whitespace(&self) -> bool {
        match self {
            TokenKind::WHITESPACE => true,
            _ => false,
        }
    }
}
