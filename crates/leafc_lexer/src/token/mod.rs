#![allow(dead_code)]
#![doc = include_str!("../../TOKEN.md")]

mod group;

use derive_more::Display;
use logos::Logos;

#[allow(non_camel_case_types)]
#[derive(Debug, Logos, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord, Display)]
pub enum TokenKind {
    #[doc = include_str!("../../../../docs/src/chapter_1/lexical_structure/tokens/general/LEXICAL_ERROR.md")]
    #[error]
    // #[regex(r"[ \t\f]+", logos::skip)]
    ERROR,

    #[doc = include_str!("../../../../docs/src/chapter_1/lexical_structure/tokens/general/WHITESPACE.md")]
    #[regex(r"[ \t\f\n]+")]
    WHITESPACE,

    #[doc = include_str!("../../../../docs/src/chapter_1/lexical_structure/tokens/general/IDENTIFIER.md")]
    #[regex(r"_?[\p{XID_Start}\p{Emoji_Presentation}][\p{XID_Continue}\p{Emoji_Presentation}]*")]
    IDENTIFIER,

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Comments
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    #[doc = include_str!("../../../../docs/src/chapter_1/lexical_structure/tokens/comments/COMMENT.md")]
    #[regex("//[^\r\n]*", priority = 2)]
    #[regex("//[^\n]*", priority = 1)]
    COMMENT,

    #[doc = include_str!("../../../../docs/src/chapter_1/lexical_structure/tokens/comments/DOC_COMMENT.md")]
    #[regex("///[^\r\n]*", priority = 4)]
    #[regex("///[^\n]*", priority = 3)]
    DOC_COMMENT,

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Literals
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    #[doc = include_str!("../../../../docs/src/chapter_1/lexical_structure/tokens/literals/RUNE.md")]
    #[regex("b?'[^']*'")]
    RUNE,

    #[doc = include_str!("../../../../docs/src/chapter_1/lexical_structure/tokens/literals/STRING.md")]
    #[regex(r#"b?"(\\.|[^\\"])*""#)]
    STRING,

    #[doc = include_str!("../../../../docs/src/chapter_1/lexical_structure/tokens/literals/RAW_STRING.md")]
    #[regex(r#"r#[^#]*#"#)]
    RAW_STRING,

    #[doc = include_str!("../../../../docs/src/chapter_1/lexical_structure/tokens/literals/INTEGER.md")]
    #[regex(
        "[+-]?[0-9][0-9_]*(u8|i8|u16|i16|u32|i32|u64|i64|u128|i128|usize|isize)?",
        priority = 1
    )] // decimal
    #[regex("[+-]?(0b|0B)[0-1][0-1_]*(u8|i8|u16|i16|u32|i32|u64|i64|u128|i128|usize|isize)?")] // binary
    #[regex("[+-]?(0o|0O)[0-7][0-7_]*(u8|i8|u16|i16|u32|i32|u64|i64|u128|i128|usize|isize)?")] // octal
    #[regex("[+-]?(0x|0X)[0-9a-fA-F][0-9a-fA-F_]*(u8|i8|u16|i16|u32|i32|u64|i64|u128|i128|usize|isize)?",
    priority = 1
    )]
    // hexadecimal
    INTEGER,

    #[doc = include_str!("../../../../docs/src/chapter_1/lexical_structure/tokens/literals/FLOAT.md")]
    #[regex(
        r#"[+-]?([0-9][0-9_]*)?\.([0-9][0-9_]*)?([eE][+-]?[0-9][0-9_]*)?(f32|f64)?"#,
        priority = 2
    )] // decimal
    #[regex(
        r#"[+-]?(0x|0X)[0-9a-fA-F][0-9a-fA-F_]*\.[0-9a-fA-F][0-9a-fA-F_]*([pP][+-]?[0-9][0-9_]?)?(f32|f64)?"#,
        priority = 2
    )] // hexadecimal
    #[regex(r#"[+-]?0b[0-1][0-1_]*\.[0-1][0-1_]*([pP][+-]?[0-9][0-9_]?)?(f32|f64)?"#)]
    // binary
    #[regex(r#"[+-]?0o[0-7][0-7_]*\.[0-7][0-7_]*([pP][+-]?[0-9][0-9_]?)?(f32|f64)?"#)] // octal
    FLOAT,

    #[doc = include_str!("../../../../docs/src/chapter_1/lexical_structure/tokens/literals/LIFETIME.md")]
    #[regex(r#"'[\p{XID_Start}\p{Emoji_Presentation}][\p{XID_Continue}\p{Emoji_Presentation}]*"#)]
    LIFETIME,

    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Superscript literals
    //////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    /// A **superscript integer** is a sequence of one or more decimal digits representing a non-negative
    /// integer value. An **optional prefix** sets the base of the integer: `0o` for octal, `0x` or
    /// `0X` for hexadecimal, and `0b` or `0B` for binary.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// foo := 2¹²
    /// assert_eq!(foo, 4096)
    /// ```
    #[regex("[⁺⁻]?[⁰¹²³⁴⁵⁶⁷⁸⁹][⁰¹²³⁴⁵⁶⁷⁸⁹_]*")] // decimal
    #[regex("[⁺⁻]?⁰ᵇ[⁰¹][⁰¹_]*")] // binary
    #[regex("[⁺⁻]?ᵒ⁰[⁰¹²³⁴⁵⁶⁷][⁰¹²³⁴⁵⁶⁷_]*")] // octal
    #[regex("[⁺⁻]?⁰ˣ[⁰¹²³⁴⁵⁶⁷⁸⁹ᴬᴮᶜᴰᴱᶠ][⁰¹²³⁴⁵⁶⁷⁸⁹ᴬᴮᶜᴰᴱᶠ_]*")] // hexadecimal
    INTEGER_SUP,

    /// A **superscript floating point number** is a sequence of decimal digits representing a floating
    /// point value. An **optional prefix** sets the base of the number: `0o` for octal, `0x` or `0X`
    ///
    /// # Examples
    ///
    /// ```ignore
    /// foo := 2¹²·⁵
    /// assert_eq!(foo, 5792.61875148)
    /// ```
    #[regex("[⁺⁻]?[⁰¹²³⁴⁵⁶⁷⁸⁹][⁰¹²³⁴⁵⁶⁷⁸⁹_]*·([⁰¹²³⁴⁵⁶⁷⁸⁹][⁰¹²³⁴⁵⁶⁷⁸⁹_]*)?")] // decimal
    #[regex("[⁺⁻]?⁰ᵇ[⁰¹][⁰¹_]*·[⁰¹][⁰¹_]*")] // binary
    #[regex("[⁺⁻]?ᵒ⁰[⁰¹²³⁴⁵⁶⁷][⁰¹²³⁴⁵⁶⁷_]*·[⁰¹²³⁴⁵⁶⁷][⁰¹²³⁴⁵⁶⁷_]*")] // octal
    #[regex(
        "[⁺⁻]?⁰ˣ[⁰¹²³⁴⁵⁶⁷⁸⁹ᴬᴮᶜᴰᴱᶠ][⁰¹²³⁴⁵⁶⁷⁸⁹ᴬᴮᶜᴰᴱᶠ_]*·[⁰¹²³⁴⁵⁶⁷⁸⁹ᴬᴮᶜᴰᴱᶠ][⁰¹²³⁴⁵⁶⁷⁸⁹ᴬᴮᶜᴰᴱᶠ_]*"
    )] // hexadecimal
    FLOAT_SUP,

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Mathematical constants, functions, and operators
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    #[doc = include_str!("../../../../docs/src/chapter_1/lexical_structure/tokens/mathematical_symbols/PI.md")]
    #[token("pi")]
    #[token("π")]
    #[token("𝜋")]
    #[token("𝛑")]
    #[token("𝝅")]
    #[token("𝞹")]
    PI,

    #[doc = include_str!("../../../../docs/src/chapter_1/lexical_structure/tokens/mathematical_symbols/EULER.md")]
    #[token("𝑒")]
    #[token("euler")]
    EULER,

    #[doc = include_str!("../../../../docs/src/chapter_1/lexical_structure/tokens/mathematical_symbols/PHI.md")]
    #[token("phi")]
    #[token("golden")]
    #[token("φ")]
    #[token("𝜙")]
    #[token("𝛗")]
    #[token("𝝓")]
    PHI,

    #[doc = include_str!("../../../../docs/src/chapter_1/lexical_structure/tokens/mathematical_symbols/TAU.md")]
    #[token("tau")]
    #[token("τ")]
    #[token("𝜏")]
    #[token("𝛕")]
    #[token("𝝉")]
    TAU,

    #[doc = include_str!("../../../../docs/src/chapter_1/lexical_structure/tokens/mathematical_symbols/CATALAN.md")]
    #[token("catalan")]
    #[token("K")]
    CATALAN,

    #[doc = include_str!("../../../../docs/src/chapter_1/lexical_structure/tokens/mathematical_symbols/EULERGAMMA.md")]
    #[token("γ")]
    #[token("𝛾")]
    #[token("eulergamma")]
    #[token("eulermascheroni")]
    EULERGAMMA,

    #[doc = include_str!("../../../../docs/src/chapter_1/lexical_structure/tokens/mathematical_symbols/INFINITY.md")]
    #[regex("[+-]?∞(16|32)?", priority = 2)]
    #[regex("[+-]?(inf|Inf|INF)(16|32)?", priority = 2)] // infinity
    INF,

    #[doc = include_str!("../../../../docs/src/chapter_1/lexical_structure/tokens/mathematical_symbols/NAN.md")]
    #[regex("[+-]?(nan|NaN|NAN)(16|32)?")]
    NAN,

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Reserved keywords (e.g. `export`, `final`, `throw`, etc.) are not allowed
    // as identifiers.
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    /// The **reseved keyword** `abstract`.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // reserved keyword (not allowed as identifier, however, it is not used)
    /// abstract
    /// abstracto
    /// abstrait
    /// abstrakt
    /// abstrato
    /// astratto
    /// abstrakti
    /// абстрактный
    /// 抽象
    /// 抽象的
    /// 추상
    /// dhahania
    /// ```
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

    /// The **reserved keyword** `is`.
    #[token("is")]
    IS_KW,

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Keywords (e.g. `as`, `if`, `else`, etc.) are not allowed as identifiers.
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    /// The **keyword** `and`.
    #[token("and")] // English
    #[token("y")] // Spanish
    #[token("et")] // French
    #[token("und")] // German
    #[token("e")] // Portuguese, Italian
    #[token("en")] // Dutch
    #[token("och")] // Swedish
    #[token("og")] // Danish, Norwegian
    #[token("ja")] // Finnish
    #[token("и")] // Russian
    #[token("と")] // Japanese
    #[token("和")] // Chinese
    #[token("및")] // Korean
    #[token("na")] // Swahili
    AND_KW,

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

    /// The **keyword** `if`.
    #[token("if")] // English
    #[token("si")] // Spanish, French
    #[token("indien")] // Dutch
    #[token("om")] // Swedish
    #[token("hvis")] // Danish, Norwegian
    #[token("jos")] // Finnish
    #[token("если")] // Russian
    #[token("もし")] // Japanese
    #[token("如果")] // Chinese
    #[token("만약")] // Korean
    #[token("ikiwa")] // Swahili
    IF_KW,

    /// The **keyword** `impl`.
    #[token("impl")] // English
    #[token("implementos")] // Spanish
    #[token("met en oeuvre")] // French
    #[token("implementeert")] // Dutch
    #[token("implementera")] // Swedish
    #[token("implementerer")] // Norwegian
    #[token("implemento")] // Italian
    #[token("implementação")] // Portuguese
    #[token("implementointi")] // Finnish
    #[token("имплементация")] // Russian
    #[token("実装")] // Japanese
    #[token("实现")] // Chinese
    #[token("구현")] // Korean
    #[token("zana")] // Swahili
    IMPL_KW,

    // The **reserved** keyword `import`.
    #[token("import")] // English
    #[token("importar")] // Spanish
    #[token("importer")] // French
    #[token("importeren")] // Dutch
    #[token("importera")] // Swedish
    #[token("importere")] // Norwegian
    #[token("importare")] // Italian
    #[token("importação")] // Portuguese
    #[token("tuonti")] // Finnish
    #[token("импорт")] // Russian
    #[token("インポート")] // Japanese
    #[token("导入")] // Chinese
    #[token("kuagiza")] // Swahili
    IMPORT_KW,

    /// The **keyword** `in`.
    ///
    /// **NOTE**: Since `in` directly translates to `i` in Swedish, Danish, and Norwegian, we use the
    /// we don't actually use the `i` keyword in those languages and instead use `in` such that
    /// `i` can be **used as a variable name** commonly used for _indexing and iteration_.
    #[token("in")] // English, German, Dutch, Finnish
    #[token("dentro de")] // Spanish
    #[token("dans")] // French
    #[token("в")] // Russian
    #[token("の中で")] // Japanese
    #[token("在")] // Chinese
    #[token("에서")] // Korean
    #[token("ndani")] // Swahili
    IN_KW,

    /// The **reserved keyword** `let`.
    #[token("let")] // English
    #[token("dejar")] // Spanish
    #[token("laisser")] // French
    #[token("laten")] // Dutch
    #[token("låta")] // Swedish
    #[token("laat")] // Danish
    #[token("la")] // Norwegian
    #[token("lasciare")] // Italian
    #[token("deixar")] // Portuguese
    #[token("jättää")] // Finnish
    #[token("пусть")] // Russian
    #[token("させる")] // Japanese
    #[token("让")] // Chinese
    #[token("놔두다")] // Korean
    #[token("acha")] // Swahili
    LET_KW,

    /// The **keyword** `loop`.
    #[token("loop")] // English
    #[token("bucle")] // Spanish
    #[token("boucle")] // French
    #[token("lussen")] // Dutch
    #[token("slinga")] // Swedish
    #[token("sløjfe")] // Danish
    #[token("løkke")] // Norwegian
    #[token("ciclo continuo")] // Italian
    #[token("laço")] // Portuguese
    #[token("silmukka")] // Finnish
    #[token("петля")] // Russian
    #[token("ループ")] // Japanese
    #[token("循环")] // Chinese
    #[token("반복")] // Korean
    #[token("kitanzi")] // Swahili
    LOOP_KW,

    /// The **keyword** `match`.
    #[token("match")] // English
    #[token("partido")] // Spanish
    #[token("correspondre")] // French
    #[token("matchen")] // Dutch
    #[token("matcha")] // Swedish
    #[token("matche")] // Danish, Norwegian
    #[token("partita")] // Italian
    #[token("partida")] // Portuguese
    #[token("ottelu")] // Finnish
    #[token("совпадение")] // Russian
    #[token("マッチ")] // Japanese
    #[token("匹配")] // Chinese
    #[token("일치")] // Korean
    #[token("mechi")] // Swahili
    MATCH_KW,

    /// The **keyword** `missing`.
    #[regex("missing|Missing")] // English
    #[regex("faltante|Faltante")] // Spanish
    #[regex("manquant|Manquant")] // French
    #[regex("missend|Missend")] // Dutch
    #[regex("fehlen|Fehlen")] // German
    #[regex("saknas|Saknas")] // Swedish
    #[regex("mangler|Mangler")] // Danish
    #[regex("savnet|Savnet")] // Norwegian
    #[regex("mancante|Mancante")] // Italian
    #[regex("ausente|Ausente")] // Portuguese
    #[regex("puuttuu|Puuttuu")] // Finnish
    #[regex("отсутствует|Отсутствует")] // Russian
    #[token("欠けている")] // Japanese
    #[token("缺失")] // Chinese
    #[token("누락")] // Korean
    #[regex("hakuna|Hakuna")] // Swahili
    MISSING_KW,

    /// The **keyword** `mod`.
    #[token("mod")] // English
    #[token("module")] // French
    #[token("moduul")] // Dutch
    #[token("modul")] // Danish, Norwegian, Swedish, German
    #[token("modulo")] // Italian
    #[token("módulo")] // Spanish, Portuguese
    #[token("moduuli")] // Finnish
    #[token("модуль")] // Russian
    #[token("モジュール")] // Japanese
    #[token("模块")] // Chinese
    #[token("모듈")] // Korean
    #[token("moduli")] // Swahili
    MOD_KW,

    /// The **keyword** `move`.
    #[token("move")] // English
    #[token("mover")] // Spanish, Portuguese
    #[token("déplacer")] // French
    #[token("verplaatsen")] // Dutch
    #[token("flytta")] // Swedish
    #[token("flytte")] // Danish, Norwegian
    #[token("muovere")] // Italian
    #[token("siirtää")] // Finnish
    #[token("перемещение")] // Russian
    #[token("移動")] // Japanese
    #[token("移动")] // Chinese
    #[token("이동")] // Korean
    #[token("hamisha")] // Swahili
    MOVE_KW,

    /// The **keyword** `mut`.
    #[token("mut")] // English
    #[token("mudable")] // Spanish
    #[token("mutable")] // French
    #[token("veranderlijk")] // Dutch
    #[token("muterbar")] // Swedish, Danish
    #[token("endringsbar")] // Norwegian
    #[token("mutabile")] // Italian
    #[token("mutável")] // Portuguese
    #[token("muuttuva")] // Finnish
    #[token("мутабельный")] // Russian
    #[token("ミュータブル")] // Japanese
    #[token("可变")] // Chinese
    #[token("변경 가능한")] // Korean
    #[token("mabadiliko")] // Swahili
    MUT_KW,

    /// The **keyword** `not`.
    #[token("not")] // English
    #[token("no")] // Spanish
    #[token("ne")] // French
    #[token("niet")] // Dutch
    #[token("nicht")] // German
    #[token("inte")] // Swedish
    #[token("ikke")] // Danish, Norwegian
    #[token("non")] // Italian
    #[token("não")] // Portuguese
    #[token("ei")] // Finnish
    #[token("не")] // Russian
    #[token("ない")] // Japanese
    #[token("不")] // Chinese
    #[token("아니")] // Korean
    #[token("sivyo")] // Swahili
    NOT_KW,

    /// The **keyword** `or`.
    #[token("or")] // English
    #[token("ou")] // French, Portuguese
    #[token("oder")] // German
    #[token("of")] // Dutch
    #[token("eller")] // Swedish, Danish, Norwegian
    #[token("o")] // Italian, Spanish
    #[token("tai")] // Finnish
    #[token("или")] // Russian
    #[token("または")] // Japanese
    #[token("或")] // Chinese
    #[token("또는")] // Korean
    #[token("au")] // Swahili
    OR_KW,

    #[token("pkg")] // English
    #[token("paquete")] // Spanish
    #[token("paquet")] // French
    #[token("pakket")] // Dutch
    #[token("paket")] // Swedish, Danish, Norwegian
    #[token("pacchetto")] // Italian
    #[token("pacote")] // Portuguese
    #[token("paketti")] // Finnish
    #[token("пакет")] // Russian
    #[token("パッケージ")] // Japanese
    #[token("包")] // Chinese
    #[token("패키지")] // Korean
    #[token("vifurushi")] // Swahili
    PACKAGE_KW,

    /// The **keyword** `pub`.
    #[token("pub")] // English
    #[token("público")] // Spanish, Portuguese
    #[token("publique")] // French
    #[token("publiek")] // Dutch
    #[token("publik")] // Swedish, Danish, Norwegian
    #[token("pubblico")] // Italian
    #[token("julkinen")] // Finnish
    #[token("публичный")] // Russian
    #[token("パブリック")] // Japanese
    #[token("公共")] // Chinese
    #[token("공용")] // Korean
    #[token("umma")] // Swahili
    PUB_KW,

    /// The **keyword** `return`.
    #[token("return")] // English
    #[token("regreso")] // Spanish
    #[token("retour")] // French
    #[token("terug")] // Dutch
    #[token("retur")] // Danish, Norwegian, Swedish
    #[token("ritorno")] // Italian
    #[token("retorno")] // Portuguese
    #[token("paluu")] // Finnish
    #[token("возврат")] // Russian
    #[token("リターン")] // Japanese
    #[token("返回")] // Chinese
    #[token("반환")] // Korean
    #[token("kurudi")] // Swahili
    RETURN_KW,

    /// The **keyword** `self`.
    #[token("self")] // English
    #[token("se")] // Spanish
    #[token("soi")] // French
    #[token("zelf")] // Dutch
    #[token("själv")] // Swedish
    #[token("selv")] // Danish, Norwegian
    #[token("sé")] // Italian
    #[token("auto")] // Portuguese
    #[token("itse")] // Finnish
    #[token("сам")] // Russian
    #[token("セルフ")] // Japanese
    #[token("自身")] // Chinese
    #[token("자기 자신")] // Korean
    #[token("mwenyewe")] // Swahili
    SELF_VALUE_KW,

    /// The **keyword** `Self`.
    #[token("Self")] // English
    #[token("Se")] // Spanish
    #[token("Soi")] // French
    #[token("Zelf")] // Dutch
    #[token("Själv")] // Swedish
    #[token("Selv")] // Danish, Norwegian
    #[token("Sé")] // Italian
    #[token("Auto")] // Portuguese
    #[token("Itse")] // Finnish
    #[token("Сам")] // Russian
    #[token("セルフタイプ")] // Japanese
    #[token("自型")] // Chinese
    #[token("자기 유형")] // Korean
    #[token("Mwenyewe")] // Swahili
    SELF_TYPE_KW,

    /// The **keyword** `static`.
    #[token("static")] // English
    #[token("estático")] // Spanish, Portuguese
    #[token("statique")] // French
    #[token("statisch")] // Dutch
    #[token("statisk")] // Swedish, Danish, Norwegian
    #[token("statico")] // Italian
    #[token("staattinen")] // Finnish
    #[token("статический")] // Russian
    #[token("スタティック")] // Japanese
    #[token("静态")] // Chinese
    #[token("정적")] // Korean
    #[token("stati")] // Swahili
    STATIC_KW,

    /// The **keyword** `struct`.
    #[token("struct")] // English
    #[token("estructura")] // Spanish
    #[token("structure")] // French
    #[token("structuur")] // Dutch
    #[token("struktur")] // Swedish, Danish, Norwegian
    #[token("struttura")] // Italian
    #[token("estrutura")] // Portuguese
    #[token("rakenne")] // Finnish
    #[token("структура")] // Russian
    #[token("ストラクチャ")] // Japanese
    #[token("结构体")] // Chinese
    #[token("구조체")] // Korean
    #[token("mifumo")] // Swahili
    STRUCT_KW,

    /// The **keyword** `super`.
    #[token("super")] // English
    #[token("supérieur")] // French
    #[token("superieur")] // Dutch
    #[token("superior")] // Danish, Norwegian, Swedish, Portuguese, Spanish
    #[token("superiore")] // Italian
    #[token("ylhäältä")] // Finnish
    #[token("супер")] // Russian
    #[token("スーパー")] // Japanese
    #[token("超级")] // Chinese
    #[token("슈퍼")] // Korean
    #[token("juu")] // Swahili
    SUPER_KW,

    /// The **keyword** `trait`.
    #[token("trait")] // English, French
    #[token("rasgo")] // Spanish
    #[token("eigenschap")] // Dutch
    #[token("egenskap")] // Danish, Norwegian, Swedish
    #[token("tratto")] // Italian
    #[token("característica")] // Portuguese
    #[token("piirre")] // Finnish
    #[token("характеристика")] // Russian
    #[token("トレイト")] // Japanese
    #[token("特征")] // Chinese
    #[token("특성")] // Korean
    #[token("tabia")] // Swahili
    TRAIT_KW,

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

    /// The **keyword** `type`.
    #[token("type")] // English, Dutch
    #[token("taper")] // French
    #[token("tipo")] // Spanish, Portuguese, Italian
    #[token("typ")] // Swedish, Danish, Norwegian, German
    #[token("tyyppi")] // Finnish
    #[token("тип")] // Russian
    #[token("タイプ")] // Japanese
    #[token("类型")] // Chinese
    #[token("유형")] // Korean
    #[token("aina")] // Swahili
    TYPE_KW,

    /// The **keyword** `unsafe`.
    #[token("unsafe")] // English
    #[token("inseguro")] // Spanish, Portuguese
    #[token("insécurisé")] // French
    #[token("unsicher")] // German
    #[token("onveilig")] // Dutch
    #[token("osäker")] // Swedish
    #[token("usikker")] // Danish, Norwegian
    #[token("insicuro")] // Italian
    #[token("epävarma")] // Finnish
    #[token("небезопасный")] // Russian
    #[token("アンセーフ")] // Japanese
    #[token("不安全")] // Chinese
    #[token("불안전")] // Korean
    #[token("haramu")] // Swahili
    UNSAFE_KW,

    /// The **keyword** `use`.
    #[token("use")] // English
    #[token("utilizar")] // Spanish
    #[token("utiliser")] // French
    #[token("gebruiken")] // Dutch
    #[token("använda")] // Swedish
    #[token("bruke")] // Norwegian
    #[token("benytte")] // Danish
    #[token("usare")] // Italian
    #[token("usar")] // Portuguese
    #[token("käyttää")] // Finnish
    #[token("использовать")] // Russian
    #[token("使用する")] // Japanese
    #[token("使用")] // Chinese
    #[token("사용")] // Korean
    #[token("tumia")] // Swahili
    USE_KW,

    /// The **keyword** `where`.
    #[token("where")] // English
    #[token("où")] // French
    #[token("dónde")] // Spanish
    #[token("wo")] // German
    #[token("waarin")] // Dutch
    #[token("hvor")] // Danish, Norwegian
    #[token("var")] // Swedish
    #[token("dove")] // Italian
    #[token("onde")] // Portuguese
    #[token("missä")] // Finnish
    #[token("где")] // Russian
    #[token("どこ")] // Japanese
    #[token("哪里")] // Chinese
    #[token("어디")] // Korean
    #[token("wapi")] // Swahili
    WHERE_KW,

    /// The **keyword** `while`.
    #[token("while")] // English
    #[token("mientras")] // Spanish
    #[token("tant que")] // French
    #[token("terwijl")] // Dutch
    #[token("medan")] // Swedish
    #[token("mens")] // Danish, Norwegian
    #[token("mentre")] // Italian
    #[token("enquanto")] // Portuguese
    #[token("während")] // German
    #[token("samalla")] // Finnish
    #[token("пока")] // Russian
    #[token("間")] // Japanese
    #[token("当")] // Chinese
    #[token("동안")] // Korean
    #[token("wakati")] // Swahili
    WHILE_KW,

    /// The **keyword** `yield`.
    #[token("yield")] // English
    #[token("rendement")] // French, Dutch
    #[token("rendimiento")] // Spanish
    #[token("avkastning")] // Swedish, Norwegian
    #[token("afkastning")] // Danish
    #[token("rendimento")] // Italian, Portuguese
    #[token("rendite")] // German
    #[token("tuotto")] // Finnish
    #[token("доход")] // Russian
    #[token("収益")] // Japanese
    #[token("收益")] // Chinese
    #[token("수익")] // Korean
    #[token("kupato")] // Swahili
    YIELD_KW,

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Punctuation
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    /// A **plus sign** `+`.
    ///
    /// **Usage**:
    ///
    /// * **Addition**,
    /// * **Trait Bounds**,
    /// * _potentially_ **Macro Kleene Matcher**.
    #[token("+")]
    PLUS,

    /// A **minus sign** `-`.
    ///
    /// **Usage**:
    ///
    /// * **Subtraction**,
    /// * **Negation**.
    #[token("-")]
    MINUS,

    /// A **star** `*`.
    ///
    /// **Usage**:
    ///
    /// * **Multiplication**,
    /// * **Dereference**,
    /// * _potentially_ **Macro Kleene Matcher**,
    /// * **Use wildcards**.
    #[token("*")]
    STAR,

    /// A **forward slash** `/`.
    ///
    /// **Usage**:
    ///
    /// * **Division**.
    #[token("/")]
    SLASH,

    /// A **percent sign** `%`.
    ///
    /// **Usage**:
    ///
    /// * **Remainder**.
    #[token("%")]
    PERCENT,

    /// A **caret** `^`.
    ///
    /// **Usage**:
    ///
    /// * **Bitwise and Logical XOR**.
    #[token("^")]
    CARET,

    /// A **bang** `!`.
    ///
    /// **Usage**:
    ///
    /// * **Bitwise and Logical NOT**,
    /// * **Macro Call**.
    /// * _potentially_ **Inner Attributes**,
    /// * _potentially_ **Never Type**,
    /// * _potentially_  **Negative impls**
    #[token("!")]
    BANG,

    /// An **ampersand** `&`.
    ///
    /// **Usage**:
    ///
    /// * **Bitwise and Logical AND**,
    /// * **Reference**,
    /// * **Borrow**,
    /// * **Reference Patterns**.
    #[token("&")]
    AMPERSAND,

    /// A **pipe** `|`.
    ///
    /// **Usage**:
    ///
    /// * **Bitwise and Logical OR**,
    /// * **Union**.
    /// * **Patterns in match, if let, and while let**.
    /// * _potentially_ **closures**.
    #[token("|")]
    PIPE,

    /// A **double ampersand** `&&`.
    ///
    /// **Usage**:
    ///
    /// * **Lazy AND**.
    /// * **Borrow**.
    /// * **References**.
    /// * **Reference patterns**.
    #[token("&&")]
    DOUBLE_AMPERSAND,

    /// A **double pipe** `||`.
    ///
    /// **Usage**:
    ///
    /// * **Lazy OR**.
    /// * _potentially_ **closures**.
    #[token("||")]
    DOUBLE_PIPE,

    /// A **shift left** `<<`.
    ///
    /// **Usage**:
    ///
    /// * **Bitwise Shift Left**.
    /// * **Nested Generics**.
    #[token("<<")]
    SHL,

    /// A **shift right** `>>`.
    ///
    /// **Usage**:
    ///
    /// * **Bitwise Shift Right**.
    /// * **Nested Generics**.
    #[token(">>")]
    SHR,

    /// A **plus equals** `+=`.
    ///
    /// **Usage**:
    ///
    /// * **Addition Assignment**.
    #[token("+=")]
    PLUS_EQ,

    /// A **minus equals** `-=`.
    ///
    /// **Usage**:
    ///
    /// * **Subtraction Assignment**.
    #[token("-=")]
    MINUS_EQ,

    /// A **star equals** `*=`.
    ///
    /// **Usage**:
    ///
    /// * **Multiplication Assignment**.
    #[token("*=")]
    STAR_EQ,

    /// A **forward slash equals** `/=`.
    ///
    /// **Usage**:
    ///
    /// * **Division Assignment**.
    #[token("/=")]
    SLASH_EQ,

    /// A **percent equals** `%=`.
    ///
    /// **Usage**:
    ///
    /// * **Remainder Assignment**.
    #[token("%=")]
    PERCENT_EQ,

    /// A **caret equals** `^=`.
    ///
    /// **Usage**:
    ///
    /// * **Bitwise XOR Assignment**.
    #[token("^=")]
    CARET_EQ,

    /// A **ampersand equals** `&=`.
    ///
    /// **Usage**:
    ///
    /// * **Bitwise AND Assignment**.
    #[token("&=")]
    AMPERSAND_EQ,

    /// A **pipe equals** `|=`.
    ///
    /// **Usage**:
    ///
    /// * **Bitwise OR Assignment**.
    #[token("|=")]
    PIPE_EQ,

    /// A **shift left equals** `<<=`.
    ///
    /// **Usage**:
    ///
    /// * **Bitwise Shift Left Assignment**.
    #[token("<<=")]
    SHL_EQ,

    /// A **shift right equals** `>>=`.
    ///
    /// **Usage**:
    ///
    /// * **Bitwise Shift Right Assignment**.
    #[token(">>=")]
    SHR_EQ,

    /// An **equal sign** `=`.
    ///
    /// **Usage**:
    ///
    /// * **Assignment**.
    /// * **Attributes**.
    #[token("=")]
    EQ,

    /// A **double equal sign** `==`.
    ///
    /// **Usage**:
    ///
    /// * **Equality**.
    #[token("==")]
    EQEQ,

    /// A **not equal sign** `!=`.
    ///
    /// **Usage**:
    ///
    /// * **Inequality**.
    #[token("!=")]
    NE,

    /// A **greater than sign** `>`.
    ///
    /// **Usage**:
    ///
    /// * **Greater than**.
    /// * **Generics**.
    /// * **Paths**.
    #[token(">")]
    GT,

    /// A **less than sign** `<`.
    ///
    /// **Usage**:
    /// * **Less than**.
    /// * **Generics**.
    /// * **Paths**.
    #[token("<")]
    LT,

    /// A **greater than or equal to sign** `>=`.
    ///
    /// **Usage**:
    /// * **Greater than or equal to**.
    /// * **Generics**.
    #[token(">=")]
    GE,

    /// A **less than or equal to sign** `<=`.
    ///
    /// **Usage**:
    ///
    /// * **Less than or equal to**.
    #[token("<=")]
    LE,

    /// An **at sign** `@`.
    ///
    /// **Usage**:
    ///
    /// * **Subpattern binding**.
    #[token("@")]
    AT,

    /// An **underscore** `_`.
    ///
    /// **Usage**:
    ///
    /// * **Wildcard patterns**.
    /// * **Inferred types**.
    /// * **Unnamed items in constants**.
    /// * **extern packages**.
    /// * **use declarations**.
    /// * **destructuring assignment**.
    #[token("_")]
    UNDERSCORE,

    /// A **dot** `.`.
    ///
    /// **Usage**:
    ///
    /// * **Field access**.
    /// * **Tuple index**.
    #[token(".", priority = 1)]
    DOT,

    /// A **dot dot** `..`.
    ///
    /// **Usage**:
    ///
    /// * **Range**.
    /// * **Struct expressions**.
    /// * **Patterns**.
    /// * **Range Patterns**.
    #[token("..")]
    DOTDOT,

    /// A **dot dot equals** `..=`.
    ///
    /// **Usage**:
    ///
    /// * **Inclusive Range**.
    /// * **Range Patterns**.
    #[token("..=")]
    DOTDOTEQ,

    /// A **comma** `,`.
    ///
    /// **Usage**:
    ///
    /// * **Separators**.
    #[token(",")]
    COMMA,

    /// A **semicolon** `;`.
    ///
    /// **Usage**:
    ///
    /// * **Terminator for various items and statements**.
    /// * **Array types**.
    #[token(";")]
    SEMICOLON,

    /// A **colon** `:`.
    ///
    /// **Usage**:
    ///
    /// * **Various separators**.
    #[token(":")]
    COLON,

    /// A **path separator** `::`.
    ///
    /// **Usage**:
    ///
    /// * **Path separator**.
    #[token("::")]
    PATHSEP,

    /// A **right arrow** `->`.
    ///
    /// **Usage**:
    ///
    /// * **Function return type**.
    /// * **Closure return type**.
    /// * **Function pointer type**.
    #[token("->")]
    RARROW,

    /// A **fat arrow** `=>`.
    ///
    /// **Usage**:
    ///
    /// * **Match arms**.
    /// * **Macros**.
    #[token("=>")]
    FATARROW,

    /// A **hash** `#`.
    ///
    /// **Usage**:
    ///
    /// * **Attributes**.
    #[token("#")]
    HASH,

    /// A **dollar** `$`.
    ///
    /// **Usage**:
    ///
    /// * **Macros**.
    #[token("$")]
    DOLLAR,

    /// A **question mark** `?`.
    ///
    /// **Usage**:
    ///
    /// * **Question mark operator**.
    /// * **Questionably sized**.
    /// * **Macro Kleene Matcher**.
    #[token("?")]
    QMARK,

    /// A **tilde** `~`.
    ///
    /// **Usage**:
    ///
    /// * **Reserved for future use**.
    #[token("~")]
    TILDE,

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Delimiters
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    /// A **left bracket** `[`.
    #[token("[")]
    L_BRACKET,

    /// A **right bracket** `]`.
    #[token("]")]
    R_BRACKET,

    /// A **left parenthesis** `(`.
    #[token("(")]
    L_PAREN,

    /// A **right parenthesis** `)`.
    #[token(")")]
    R_PAREN,

    /// A **left brace** `{`.
    #[token("{")]
    L_BRACE,

    /// A **right brace** `}`.
    #[token("}")]
    R_BRACE,

    /// A **superscript left parenthesis** `⁽`.
    ///
    /// **Usage**:
    ///
    /// * **Superscript Expressions**.
    #[token("⁽")]
    L_PAREN_SUPERSCRIPT,

    /// A **superscript right parenthesis** `⁾`.
    ///
    /// **Usage**:
    ///
    /// * **Superscript Expressions**.
    #[token("⁾")]
    R_PAREN_SUPERSCRIPT,

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Reserved tokens (Experimental)
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    #[token("<-")]
    L_ARROW,

    #[token("**")]
    DOUBLE_STAR,
}

impl TokenKind {
    pub fn is_whitespace(&self) -> bool {
        matches!(self, TokenKind::WHITESPACE)
    }

    pub fn is_comment(&self) -> bool {
        matches!(self, TokenKind::COMMENT | TokenKind::DOC_COMMENT)
    }
}
