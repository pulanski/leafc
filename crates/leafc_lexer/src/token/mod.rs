#![allow(dead_code)]
#![doc = include_str!("../../TOKEN.md")]

mod group;

use derive_more::Display;
use logos::Logos;

#[allow(non_camel_case_types)]
#[derive(Debug, Logos, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord, Display)]
pub enum TokenKind {
    #[error]
    // #[regex(r"[ \t\f]+", logos::skip)]
    ERROR,

    #[regex("//[^\r\n]*", priority = 2)]
    #[regex("//[^\n]*", priority = 1)]
    COMMENT,

    #[regex("///[^\r\n]*", priority = 4)]
    #[regex("///[^\n]*", priority = 3)]
    DOC_COMMENT,

    #[regex(r"[ \t\f\n]+")]
    WHITESPACE,

    #[regex("b?'[^']*'")]
    // #[regex("b?'\\x[0-9a-fA-F][0-9a-fA-F]'")]
    RUNE,

    #[regex(r#"b?"(\\.|[^\\"])*""#)]
    STRING,

    #[token("inf")]
    #[token("NaN")]
    #[regex(r#"[+-]?[0-9][0-9_]*\.[0-9][0-9_]*([eE][+-]?[0-9][0-9_]*)?"#)]
    #[regex(
        r#"[+-]?0x[0-9a-fA-F][0-9a-fA-F_]*\.[0-9a-fA-F][0-9a-fA-F_]*([pP][+-]?[0-9][0-9_]?)?"#
    )]
    FLOAT,

    #[regex("[+-]?[0-9][0-9_]*")]
    #[regex("[+-]?0b[0-1][0-1_]*")]
    #[regex("[+-]?0x[0-9a-fA-F][0-9a-fA-F_]*")]
    INTEGER,

    ///////////////////////////////////////////////////////////////////////////////
    // Reserved keywords (e.g. `export`, `final`, `throw`, etc.) are not allowed //
    // as identifiers.                                                           //
    ///////////////////////////////////////////////////////////////////////////////
    /// The **reseved keyword** `abstract`.
    #[token("abstract")] // English, Dutch
    #[token("abstracto")] // Spanish
    #[token("abstrait")] // French
    #[token("abstrakt")] // German, Danish, Norwegian, Swedish
    #[token("abstrato")] // Portuguese
    #[token("astratto")] // Italian
    #[token("abstrakti")] // Finnish
    #[token("абстрактный")] // Russian
    #[token("抽象")] // Japanese
    #[token("抽象的")] // Chinese
    #[token("추상")] // Korean
    #[token("dhahania")] // Swahili
    ABSTRACT_KW,

    /// The **reseved keyword** `async`.
    #[token("async")] // English
    #[token("asíncrono")] // Spanish
    #[token("asynchrone")] // French
    #[token("asynchron")] // German
    #[token("assíncrono")] // Portuguese
    #[token("asincrono")] // Italian
    #[token("asynkron")] // Danish, Norwegian, Swedish
    #[token("asynchroon")] // Dutch
    #[token("asynk")] // Finnish
    #[token("асинхронный")] // Russian
    #[token("非同期")] // Japanese
    #[token("异步")] // Chinese
    #[token("비동기")] // Korean
    #[token("isiyolingana")] // Swahili
    ASYNC_KW,

    //////////////////////////////////////////////////////////////////////////////
    // Keywords (e.g. `as`, `if`, `else`, etc.) are not allowed as identifiers. //
    //////////////////////////////////////////////////////////////////////////////
    /// The **keyword** `as`.
    #[token("as")] // English
    #[token("como")] // Spanish, Portuguese
    #[token("comme")] // French
    #[token("wie")] // German
    #[token("come")] // Italian
    #[token("als")] // Dutch
    #[token("som")] // Swedish, Danish, Norwegian
    #[token("kuten")] // Finnish
    #[token("как")] // Russian
    #[token("として")] // Japanese
    #[token("作为")] // Chinese
    #[token("로")] // Korean
    #[token("kama")] // Swahili
    AS_KW,

    /// The **keyword** `false`.
    #[regex("false|False")] // English
    #[regex("falso|Falso")] // Spanish, Italian, Portuguese
    #[regex("faux|Faux")] // French
    #[regex("falsch|Falsch")] // German
    #[regex("vals|Vals")] // Dutch
    #[regex("falsk|Falsk")] // Norwegian, Swedish, Danish
    #[regex("väärä|Väärä")] // Finnish
    #[token("ЛОЖЬ")] // Russian
    #[token("間違い")] // Japanese
    #[token("错误的")] // Chinese
    #[token("거짓")] // Korean
    #[regex("uongo|Uongo")] // Swahili
    FALSE_KW,

    /// The **keyword** `true`.
    #[regex("true|True")] // English
    #[regex("verdadero|Verdadero")] // Spanish
    #[regex("vrai|Vrai")] // French
    #[regex("wahr|Wahr")] // German
    #[regex("verdadeiro|Verdadeiro")] // Portuguese
    #[regex("vero|Vero")] // Italian
    #[regex("waar|Waar")] // Dutch
    #[regex("sann|Sann")] // Swedish
    #[regex("rigtigt|Rigtigt")] // Danish
    #[regex("ekte|Ekte")] // Norwegian
    #[regex("totta|Totta")] // Finnish
    #[regex("истинный")] // Russian
    #[token("真実")] // Japanese
    #[token("真的")] // Chinese
    #[token("진실")] // Korean
    #[token("kweli")] // Swahili
    TRUE_KW,

    #[token("const")]
    CONST,

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
    ELSE_KW,

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
        matches!(self, TokenKind::WHITESPACE)
    }

    pub fn is_comment(&self) -> bool {
        matches!(self, TokenKind::COMMENT | TokenKind::DOC_COMMENT)
    }
}

#[cfg(test)]
mod token_test_suite {
    use super::*;
    use pretty_assertions_sorted::assert_eq;
    use rstest::rstest;

    // #[rstest]
    // #[case("fn", TokenKind::FN_KW)]
    // #[case("let", TokenKind::Let)]

    // Test all keywords

    #[rstest]
    #[case("abstract")]
    #[case("abstracto")]
    #[case("abstrait")]
    #[case("abstrakt")]
    #[case("abstrato")]
    #[case("astratto")]
    #[case("abstrakti")]
    #[case("абстрактный")]
    #[case("抽象")]
    #[case("抽象的")]
    #[case("추상")]
    #[case("dhahania")]
    fn test_abstract_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::ABSTRACT_KW));
    }

    #[rstest]
    #[case("async")] // English
    #[case("asíncrono")] // Spanish
    #[case("asynchrone")] // French
    #[case("asynchron")] // German
    #[case("assíncrono")] // Portuguese
    #[case("asincrono")] // Italian
    #[case("asynkron")] // Danish, Norwegian, Swedish
    #[case("asynchroon")] // Dutch
    #[case("asynk")] // Finnish
    #[case("асинхронный")] // Russian
    #[case("非同期")] // Japanese
    #[case("异步")] // Chinese
    #[case("비동기")] // Korean
    #[case("isiyolingana")] // Swahili
    fn test_async_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::ASYNC_KW));
    }

    #[rstest]
    #[case("as")]
    #[case("como")]
    #[case("comme")]
    #[case("wie")]
    #[case("come")]
    #[case("som")]
    #[case("kuten")]
    #[case("как")]
    #[case("として")]
    #[case("作为")]
    #[case("로")]
    #[case("kama")]
    fn test_as_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::AS_KW));
    }

    #[rstest]
    #[case("true")]
    #[case("True")]
    #[case("verdadero")]
    #[case("Verdadero")]
    #[case("vrai")]
    #[case("Vrai")]
    #[case("wahr")]
    #[case("Wahr")]
    #[case("verdadeiro")]
    #[case("Verdadeiro")]
    #[case("vero")]
    #[case("Vero")]
    #[case("waar")]
    #[case("Waar")]
    #[case("sann")]
    #[case("Sann")]
    #[case("rigtigt")]
    #[case("Rigtigt")]
    #[case("ekte")]
    #[case("Ekte")]
    #[case("totta")]
    #[case("Totta")]
    #[case("sann")]
    #[case("Sann")]
    #[case("истинный")]
    #[case("真実")]
    #[case("真的")]
    #[case("진실")]
    #[case("kweli")]
    fn test_token_kind_true(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::TRUE_KW));
    }

    #[rstest]
    #[case("false")]
    #[case("False")]
    #[case("falso")]
    #[case("Falso")]
    #[case("faux")]
    #[case("Faux")]
    #[case("falsch")]
    #[case("Falsch")]
    #[case("vals")]
    #[case("Vals")]
    #[case("falsk")]
    #[case("Falsk")]
    #[case("väärä")]
    #[case("Väärä")]
    #[case("ЛОЖЬ")]
    #[case("間違い")]
    #[case("错误的")]
    #[case("거짓")]
    #[case("uongo")]
    #[case("Uongo")]
    fn test_token_kind_false(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::FALSE_KW));
    }

    // #[regex("vals|Vals")] // Dutch
    // #[regex("falsk|Falsk")] // Norwegian, Swedish, Danish
    // #[regex("väärä|Väärä")] // Finnish
    // #[token("ЛОЖЬ")] // Russian
    // #[token("間違い")] // Japanese
    // #[token("错误的")] // Chinese
    // #[token("거짓")] // Korean
    // #[regex("uongo/Uongo")] // Swahili

    // English
    // #[token("true|True")]
    // // Spanish
    // #[token("verdadero|Verdadero")]
    // // French
    // #[token("vrai|Vrai")]
    // // German
    // #[token("wahr|Wahr")]
    // // Portuguese
    // #[token("verdadeiro|Verdadeiro")]
    // // Italian
    // #[token("vero|Vero")]
    // // Dutch
    // #[token("waar|Waar")]
    // // Swedish
    // #[token("sann|Sann")]
    // // Danish
    // #[token("rigtigt|Rigtigt")]
    // // Norwegian
    // #[token("ekte|Ekte")]
    // // Finnish
    // #[token("totta|Totta")]
    // // Russian
    // #[token("истинный")]
    // // Japanese
    // #[token("真実")]
    // // Chinese
    // #[token("真的")]
    // // Korean
    // #[token("진실")]
    // TRUE,
}
