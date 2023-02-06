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

    /// The **reseved keyword** `await`.
    #[token("await")] // English
    #[token("esperar")] // Spanish
    #[token("attendre")] // French
    #[token("erwarten")] // German
    #[token("aguardam")] // Portuguese
    #[token("attendere")] // Italian
    #[token("vente")] // Danish
    #[token("avvente")] // Norwegian
    #[token("vänta")] // Swedish
    #[token("wachten")] // Dutch
    #[token("odottaa")] // Finnish
    #[token("Ждите")] // Russian
    #[token("待つ")] // Japanese
    #[token("等待")] // Chinese
    #[token("기다리다")] // Korean
    #[token("kusubiri")] // Swahili
    AWAIT_KW,

    /// The **reserved keyword** `extern`.
    #[token("extern")] // English, Danish, Norwegian, Swedish, German, Dutch
    #[token("externo")] // Spanish, Portuguese
    #[token("externe")] // French
    #[token("esterno")] // Italian
    #[token("ulkoinen")] // Finnish
    #[token("внешний")] // Russian
    #[token("外部")] // Japanese
    #[token("外部的")] // Chinese
    #[token("외부")] // Korean
    #[token("nje")] // Swahili
    EXTERN_KW,

    /// The **reserved keyword** `final`.
    #[token("final")] // English,Spanish, Portuguese, Danish, Norwegian, Swedish, German, Dutch
    #[token("finale")] // French
    #[token("finaali")] // Finnish
    #[token("конечный")] // Russian
    #[token("最終")] // Japanese
    #[token("最终")] // Chinese
    #[token("최종")] // Korean
    #[token("mwisho")] // Swahili
    FINAL_KW,

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

    /// The **keyword** `break`.
    #[token("break")] // English
    #[token("romper")] // Spanish
    #[token("casser")] // French
    #[token("brechen")] // German
    #[token("quebrar")] // Portuguese
    #[token("rompere")] // Italian
    #[token("pauze")] // Dutch
    #[token("ha sönder")] // Swedish
    #[token("pause")] // Danish
    #[token("bryte")] // Norwegian
    #[token("katkaista")] // Finnish
    #[token("прервать")] // Russian
    #[token("ブレーク")] // Japanese
    #[token("中断")] // Chinese
    #[token("중단")] // Korean
    #[token("kuvunja")] // Swahili
    BREAK_KW,

    /// The **keyword** `const`.
    #[token("const")] // English
    #[token("constante")] // Spanish, French, Portuguese, Italian, Dutch
    #[token("konstante")] // German
    #[token("konstant")] // Swedish, Danish, Norwegian
    #[token("vakio")] // Finnish
    #[token("константа")] // Russian
    #[token("定数")] // Japanese
    #[token("常量")] // Chinese
    #[token("상수")] // Korean
    #[token("mstari")] // Swahili
    CONST_KW,

    /// The **keyword** `continue`.
    #[token("continue")] // English
    #[token("continuar")] // Spanish, Portuguese
    #[token("continuer")] // French
    #[token("fortsetzen")] // German
    #[token("continuare")] // Italian
    #[token("doorgaan")] // Dutch
    #[token("fortsätta")] // Swedish
    #[token("fortsætte")] // Danish
    #[token("jatkaa")] // Finnish
    #[token("продолжать")] // Russian
    #[token("続行")] // Japanese
    #[token("继续")] // Chinese
    #[token("계속")] // Korean
    #[token("kuendelea")] // Swahili
    CONTINUE_KW,

    /// The **keyword** `do`.
    #[token("do")] // English
    #[token("hacer")] // Spanish
    #[token("faire")] // French
    #[token("machen")] // German
    #[token("fazer")] // Portuguese
    #[token("fare")] // Italian
    #[token("doen")] // Dutch
    #[token("göra")] // Swedish
    #[token("gøre")] // Danish
    #[token("gjøre")] // Norwegian
    #[token("tehdä")] // Finnish
    #[token("делать")] // Russian
    #[token("する")] // Japanese
    #[token("做")] // Chinese
    #[token("하다")] // Korean
    #[token("kufanya")] // Swahili
    DO_KW,

    /// The **keyword** `dyn`.
    #[token("dyn")] // English
    #[token("dinámico")] // Spanish
    #[token("dynamique")] // French
    #[token("dinâmico")] // Portuguese
    #[token("dinamico")] // Italian
    #[token("dynamisch")] // German, Dutch
    #[token("dynaaminen")] // Finnish
    #[token("dynamisk")] // Danish, Norwegian, Swedish
    #[token("динамический")] // Russian
    #[token("ダイナミック")] // Japanese
    #[token("动态")] // Chinese
    #[token("yenye nguvu")] // Swahili
    DYN_KW,

    /// The **keyword** `else`.
    #[token("else")] // English
    #[token("sino")] // Spanish
    #[token("sinon")] // French
    #[token("sonst")] // German
    #[token("se não")] // Portuguese
    #[token("altrimenti")] // Italian
    #[token("anders")] // Dutch
    #[token("annars")] // Swedish
    #[token("ellers")] // Danish
    #[token("muuten")] // Finnish
    #[token("иначе")] // Russian
    #[token("それ以外")] // Japanese
    #[token("否则")] // Chinese
    #[token("그렇지 않으면")] // Korean
    #[token("kama siyo")] // Swahili
    ELSE_KW,

    /// The **keyword** `enum`.
    #[token("enum")] // English
    #[token("enumera")] // Spanish, Swedish
    #[token("énumération")] // French
    #[token("enumeração")] // Portuguese
    #[token("enumerazione")] // Italian
    #[token("enummer")] // Dutch, Danish, Norwegian
    #[token("enumeraatio")] // Finnish
    #[token("перечисление")] // Russian
    #[token("列挙")] // Japanese
    #[token("枚举")] // Chinese
    #[token("열거")] // Korean
    #[token("orodha")] // Swahili
    ENUM_KW,

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

    /// The **keyword** `fn`.
    #[token("fn")] // English
    #[token("función")] // Spanish
    #[token("fonction")] // French
    #[token("funktion")] // German, Swedish, Danish
    #[token("functie")] // Dutch
    #[token("funzione")] // Italian
    #[token("função")] // Portuguese
    #[token("funksjon")] // Norwegian
    #[token("toiminto")] // Finnish
    #[token("функция")] // Russian
    #[token("関数")] // Japanese
    #[token("函数")] // Chinese
    #[token("함수")] // Korean
    #[token("fanya")] // Swahili
    FN_KW,

    /// The **keyword** `for`.
    #[token("for")] // English, Danish, Norwegian, Finnish
    #[token("para")] // Spanish
    #[token("pour")] // French
    #[token("voor")] // Dutch
    #[token("för")] // Swedish
    #[token("для")] // Russian
    #[token("のために")] // Japanese
    #[token("为")] // Chinese
    #[token("위해")] // Korean
    #[token("kwa")] // Swahili
    FOR_KW,

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

    #[regex(r"[\p{XID_Start}\p{Emoji_Presentation}][\p{XID_Continue}\p{Emoji_Presentation}]*")]
    IDENT,

    #[token("import")]
    Import,
    #[token("let")]
    Let,
    #[token("type")]
    Type,

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

    #[token("return")]
    RETURN_KW,

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
    #[case("await")] // English
    #[case("esperar")] // Spanish
    #[case("attendre")] // French
    #[case("erwarten")] // German
    #[case("aguardam")] // Portuguese
    #[case("attendere")] // Italian
    #[case("vente")] // Danish
    #[case("avvente")] // Norwegian
    #[case("vänta")] // Swedish
    #[case("wachten")] // Dutch
    #[case("odottaa")] // Finnish
    #[case("Ждите")] // Russian
    #[case("待つ")] // Japanese
    #[case("等待")] // Chinese
    #[case("기다리다")] // Korean
    #[case("kusubiri")] // Swahili
    #[test]
    fn test_await_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::AWAIT_KW));
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
    #[case("break")] // English
    #[case("romper")] // Spanish
    #[case("casser")] // French
    #[case("brechen")] // German
    #[case("quebrar")] // Portuguese
    #[case("rompere")] // Italian
    #[case("pauze")] // Dutch
    #[case("ha sönder")] // Swedish
    #[case("pause")] // Danish
    #[case("bryte")] // Norwegian
    #[case("katkaista")] // Finnish
    #[case("прервать")] // Russian
    #[case("ブレーク")] // Japanese
    #[case("中断")] // Chinese
    #[case("중단")] // Korean
    #[case("kuvunja")] // Swahili
    fn test_break_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::BREAK_KW));
    }

    #[rstest]
    #[case("const")] // English
    #[case("constante")] // Spanish, French, Portuguese, Italian, Dutch
    #[case("konstante")] // German
    #[case("konstant")] // Swedish, Danish, Norwegian
    #[case("vakio")] // Finnish
    #[case("константа")] // Russian
    #[case("定数")] // Japanese
    #[case("常量")] // Chinese
    #[case("상수")] // Korean
    #[case("mstari")] // Swahili
    fn test_const_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::CONST_KW));
    }

    #[rstest]
    #[case("continue")] // English
    #[case("continuar")] // Spanish, Portuguese
    #[case("continuer")] // French
    #[case("fortsetzen")] // German
    #[case("continuare")] // Italian
    #[case("doorgaan")] // Dutch
    #[case("fortsätta")] // Swedish
    #[case("fortsætte")] // Danish
    #[case("jatkaa")] // Finnish
    #[case("продолжать")] // Russian
    #[case("続行")] // Japanese
    #[case("继续")] // Chinese
    #[case("계속")] // Korean
    #[case("kuendelea")] // Swahili
    fn test_continue_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::CONTINUE_KW));
    }

    #[rstest]
    #[case("do")] // English
    #[case("hacer")] // Spanish
    #[case("faire")] // French
    #[case("machen")] // German
    #[case("fazer")] // Portuguese
    #[case("fare")] // Italian
    #[case("doen")] // Dutch
    #[case("göra")] // Swedish
    #[case("gøre")] // Danish
    #[case("gjøre")] // Norwegian
    #[case("tehdä")] // Finnish
    #[case("делать")] // Russian
    #[case("する")] // Japanese
    #[case("做")] // Chinese
    #[case("하다")] // Korean
    #[case("kufanya")] // Swahili
    fn test_do_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::DO_KW));
    }

    #[rstest]
    #[case("dyn")] // English
    #[case("dinámico")] // Spanish
    #[case("dynamique")] // French
    #[case("dinâmico")] // Portuguese
    #[case("dinamico")] // Italian
    #[case("dynamisch")] // German, Dutch
    #[case("dynaaminen")] // Finnish
    #[case("dynamisk")] // Danish, Norwegian, Swedish
    #[case("динамический")] // Russian
    #[case("ダイナミック")] // Japanese
    #[case("动态")] // Chinese
    #[case("yenye nguvu")] // Swahili
    fn test_dyn_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::DYN_KW));
    }

    #[rstest]
    #[case("else")] // English
    #[case("sino")] // Spanish
    #[case("sinon")] // French
    #[case("sonst")] // German
    #[case("se não")] // Portuguese
    #[case("altrimenti")] // Italian
    #[case("anders")] // Dutch
    #[case("annars")] // Swedish
    #[case("ellers")] // Danish
    #[case("muuten")] // Finnish
    #[case("иначе")] // Russian
    #[case("それ以外")] // Japanese
    #[case("否则")] // Chinese
    #[case("그렇지 않으면")] // Korean
    #[case("kama siyo")] // Swahili
    fn test_else_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::ELSE_KW));
    }

    #[rstest]
    #[case("enum")] // English
    #[case("enumera")] // Spanish, Swedish
    #[case("énumération")] // French
    #[case("enumeração")] // Portuguese
    #[case("enumerazione")] // Italian
    #[case("enummer")] // Dutch, Danish, Norwegian
    #[case("enumeraatio")] // Finnish
    #[case("перечисление")] // Russian
    #[case("列挙")] // Japanese
    #[case("枚举")] // Chinese
    #[case("열거")] // Korean
    #[case("orodha")] // Swahili
    fn test_enum_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::ENUM_KW));
    }

    #[rstest]
    #[case("extern")] // English, Danish, Norwegian, Swedish, German, Dutch
    #[case("externo")] // Spanish
    #[case("externe")] // French
    #[case("externo")] // Portuguese
    #[case("esterno")] // Italian
    #[case("ulkoinen")] // Finnish
    #[case("внешний")] // Russian
    #[case("外部")] // Japanese
    #[case("外部的")] // Chinese
    #[case("외부")] // Korean
    #[case("nje")] // Swahili
    fn test_extern_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::EXTERN_KW));
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
    fn test_false_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::FALSE_KW));
    }

    #[rstest]
    #[case("final")] // English,Spanish, Portuguese, Danish, Norwegian, Swedish, German, Dutch
    #[case("finale")] // French
    #[case("finaali")] // Finnish
    #[case("конечный")] // Russian
    #[case("最終")] // Japanese
    #[case("最终")] // Chinese
    #[case("최종")] // Korean
    #[case("mwisho")] // Swahili
    fn test_final_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::FINAL_KW));
    }

    #[rstest]
    #[case("fn")] // English
    #[case("función")] // Spanish
    #[case("fonction")] // French
    #[case("funktion")] // German, Swedish, Danish
    #[case("functie")] // Dutch
    #[case("funzione")] // Italian
    #[case("função")] // Portuguese
    #[case("funksjon")] // Norwegian
    #[case("toiminto")] // Finnish
    #[case("функция")] // Russian
    #[case("関数")] // Japanese
    #[case("函数")] // Chinese
    #[case("함수")] // Korean
    #[case("fanya")] // Swahili
    fn test_fn_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::FN_KW));
    }

    #[rstest]
    #[case("for")] // English, Danish, Norwegian, Finnish
    #[case("para")] // Spanish
    #[case("pour")] // French
    #[case("voor")] // Dutch
    #[case("för")] // Swedish
    #[case("для")] // Russian
    #[case("のために")] // Japanese
    #[case("为")] // Chinese
    #[case("위해")] // Korean
    #[case("kwa")] // Swahili
    fn test_for_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::FOR_KW));
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
