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

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Reserved keywords (e.g. `export`, `final`, `throw`, etc.) are not allowed
    // as identifiers.
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////
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

    #[regex(r"[\p{XID_Start}\p{Emoji_Presentation}][\p{XID_Continue}\p{Emoji_Presentation}]*")]
    IDENT,

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

    // Test all keywords

    #[rstest]
    #[case("and")] // English
    #[case("y")] // Spanish
    #[case("et")] // French
    #[case("und")] // German
    #[case("e")] // Portuguese, Italian
    #[case("en")] // Dutch
    #[case("och")] // Swedish
    #[case("og")] // Danish, Norwegian
    #[case("ja")] // Finnish
    #[case("и")] // Russian
    #[case("と")] // Japanese
    #[case("和")] // Chinese
    #[case("및")] // Korean
    #[case("na")] // Swahili
    fn test_and_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::AND_KW));
    }

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
    #[case("if")] // English
    #[case("si")] // Spanish, French
    #[case("indien")] // Dutch
    #[case("om")] // Swedish
    #[case("hvis")] // Danish, Norwegian
    #[case("jos")] // Finnish
    #[case("если")] // Russian
    #[case("もし")] // Japanese
    #[case("如果")] // Chinese
    #[case("만약")] // Korean
    #[case("ikiwa")] // Swahili
    fn test_if_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::IF_KW));
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
    #[case("impl")] // English
    #[case("implementos")] // Spanish
    #[case("met en oeuvre")] // French
    #[case("implementeert")] // Dutch
    #[case("implementera")] // Swedish
    #[case("implementerer")] // Norwegian
    #[case("implemento")] // Italian
    #[case("implementação")] // Portuguese
    #[case("implementointi")] // Finnish
    #[case("имплементация")] // Russian
    #[case("実装")] // Japanese
    #[case("实现")] // Chinese
    #[case("구현")] // Korean
    #[case("zana")] // Swahili
    fn test_impl_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::IMPL_KW));
    }

    #[rstest]
    #[case("import")] // English
    #[case("importar")] // Spanish
    #[case("importer")] // French
    #[case("importeren")] // Dutch
    #[case("importera")] // Swedish
    #[case("importere")] // Norwegian
    #[case("importare")] // Italian
    #[case("importação")] // Portuguese
    #[case("tuonti")] // Finnish
    #[case("импорт")] // Russian
    #[case("インポート")] // Japanese
    #[case("导入")] // Chinese
    #[case("kuagiza")] // Swahili
    fn test_import_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::IMPORT_KW));
    }

    #[rstest]
    #[case("in")] // English, German, Dutch, Finnish
    #[case("dentro de")] // Spanish
    #[case("dans")] // French
    #[case("в")] // Russian
    #[case("の中で")] // Japanese
    #[case("在")] // Chinese
    #[case("에서")] // Korean
    #[case("ndani")] // Swahili
    fn test_in_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::IN_KW));
    }

    #[rstest]
    #[case("let")] // English
    #[case("dejar")] // Spanish
    #[case("laisser")] // French
    #[case("laten")] // Dutch
    #[case("låta")] // Swedish
    #[case("laat")] // Danish
    #[case("la")] // Norwegian
    #[case("lasciare")] // Italian
    #[case("deixar")] // Portuguese
    #[case("jättää")] // Finnish
    #[case("пусть")] // Russian
    #[case("させる")] // Japanese
    #[case("让")] // Chinese
    #[case("놔두다")] // Korean
    #[case("acha")] // Swahili
    fn test_let_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::LET_KW));
    }

    #[rstest]
    #[case("loop")] // English
    #[case("bucle")] // Spanish
    #[case("boucle")] // French
    #[case("lussen")] // Dutch
    #[case("slinga")] // Swedish
    #[case("sløjfe")] // Danish
    #[case("løkke")] // Norwegian
    #[case("ciclo continuo")] // Italian
    #[case("laço")] // Portuguese
    #[case("silmukka")] // Finnish
    #[case("петля")] // Russian
    #[case("ループ")] // Japanese
    #[case("循环")] // Chinese
    #[case("반복")] // Korean
    #[case("kitanzi")] // Swahili
    fn test_loop_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::LOOP_KW));
    }

    #[rstest]
    #[case("match")] // English
    #[case("partido")] // Spanish
    #[case("correspondre")] // French
    #[case("matchen")] // Dutch
    #[case("matcha")] // Swedish
    #[case("matche")] // Danish, Norwegian
    #[case("partita")] // Italian
    #[case("partida")] // Portuguese
    #[case("ottelu")] // Finnish
    #[case("совпадение")] // Russian
    #[case("マッチ")] // Japanese
    #[case("匹配")] // Chinese
    #[case("일치")] // Korean
    #[case("mechi")] // Swahili
    fn test_match_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::MATCH_KW));
    }

    #[rstest]
    #[case("mod")] // English
    #[case("module")] // French
    #[case("moduul")] // Dutch
    #[case("modul")] // Danish, Norwegian, Swedish, German
    #[case("modulo")] // Italian
    #[case("módulo")] // Spanish, Portuguese
    #[case("moduuli")] // Finnish
    #[case("модуль")] // Russian
    #[case("モジュール")] // Japanese
    #[case("模块")] // Chinese
    #[case("모듈")] // Korean
    #[case("moduli")] // Swahili
    fn test_mod_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::MOD_KW));
    }

    #[rstest]
    #[case("move")] // English
    #[case("mover")] // Spanish, Portuguese
    #[case("déplacer")] // French
    #[case("verplaatsen")] // Dutch
    #[case("flytta")] // Swedish
    #[case("flytte")] // Danish, Norwegian
    #[case("muovere")] // Italian
    #[case("siirtää")] // Finnish
    #[case("перемещение")] // Russian
    #[case("移動")] // Japanese
    #[case("移动")] // Chinese
    #[case("이동")] // Korean
    #[case("hamisha")] // Swahili
    fn test_move_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::MOVE_KW));
    }

    #[rstest]
    #[case("mut")] // English
    #[case("mudable")] // Spanish
    #[case("mutable")] // French
    #[case("veranderlijk")] // Dutch
    #[case("muterbar")] // Swedish, Danish
    #[case("endringsbar")] // Norwegian
    #[case("mutabile")] // Italian
    #[case("mutável")] // Portuguese
    #[case("muuttuva")] // Finnish
    #[case("мутабельный")] // Russian
    #[case("ミュータブル")] // Japanese
    #[case("可变")] // Chinese
    #[case("변경 가능한")] // Korean
    #[case("mabadiliko")] // Swahili
    fn test_mut_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::MUT_KW));
    }

    #[rstest]
    #[case("or")] // English
    #[case("ou")] // French, Portuguese
    #[case("oder")] // German
    #[case("of")] // Dutch
    #[case("eller")] // Swedish, Danish, Norwegian
    #[case("o")] // Italian, Spanish
    #[case("tai")] // Finnish
    #[case("или")] // Russian
    #[case("または")] // Japanese
    #[case("或")] // Chinese
    #[case("또는")] // Korean
    #[case("au")] // Swahili
    fn test_or_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::OR_KW));
    }

    #[rstest]
    #[case("pub")] // English
    #[case("público")] // Spanish, Portuguese
    #[case("publique")] // French
    #[case("publiek")] // Dutch
    #[case("publik")] // Swedish, Danish, Norwegian
    #[case("pubblico")] // Italian
    #[case("julkinen")] // Finnish
    #[case("публичный")] // Russian
    #[case("パブリック")] // Japanese
    #[case("公共")] // Chinese
    #[case("공용")] // Korean
    #[case("umma")] // Swahili
    fn test_pub_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::PUB_KW));
    }

    #[rstest]
    #[case("return")] // English
    #[case("regreso")] // Spanish
    #[case("retour")] // French
    #[case("terug")] // Dutch
    #[case("retur")] // Danish, Norwegian, Swedish
    #[case("ritorno")] // Italian
    #[case("retorno")] // Portuguese
    #[case("paluu")] // Finnish
    #[case("возврат")] // Russian
    #[case("リターン")] // Japanese
    #[case("返回")] // Chinese
    #[case("반환")] // Korean
    #[case("kurudi")] // Swahili
    fn test_return_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::RETURN_KW));
    }

    #[rstest]
    #[case("self")] // English
    #[case("se")] // Spanish
    #[case("soi")] // French
    #[case("zelf")] // Dutch
    #[case("själv")] // Swedish
    #[case("selv")] // Danish, Norwegian
    #[case("sé")] // Italian
    #[case("auto")] // Portuguese
    #[case("itse")] // Finnish
    #[case("сам")] // Russian
    #[case("セルフ")] // Japanese
    #[case("自身")] // Chinese
    #[case("자기 자신")] // Korean
    #[case("mwenyewe")] // Swahili
    fn test_self_value_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::SELF_VALUE_KW));
    }

    #[rstest]
    #[case("Self")] // English
    #[case("Se")] // Spanish
    #[case("Soi")] // French
    #[case("Zelf")] // Dutch
    #[case("Själv")] // Swedish
    #[case("Selv")] // Danish, Norwegian
    #[case("Sé")] // Italian
    #[case("Auto")] // Portuguese
    #[case("Itse")] // Finnish
    #[case("Сам")] // Russian
    #[case("セルフタイプ")] // Japanese
    #[case("自型")] // Chinese
    #[case("자기 유형")] // Korean
    #[case("Mwenyewe")] // Swahili
    fn test_self_type_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::SELF_TYPE_KW));
    }

    #[rstest]
    #[case("static")] // English
    #[case("estático")] // Spanish, Portuguese
    #[case("statique")] // French
    #[case("statisch")] // Dutch
    #[case("statisk")] // Swedish, Danish, Norwegian
    #[case("statico")] // Italian
    #[case("staattinen")] // Finnish
    #[case("статический")] // Russian
    #[case("スタティック")] // Japanese
    #[case("静态")] // Chinese
    #[case("정적")] // Korean
    #[case("stati")] // Swahili
    fn test_static_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::STATIC_KW));
    }

    #[rstest]
    #[case("struct")] // English
    #[case("estructura")] // Spanish
    #[case("structure")] // French
    #[case("structuur")] // Dutch
    #[case("struktur")] // Swedish, Danish, Norwegian
    #[case("struttura")] // Italian
    #[case("estrutura")] // Portuguese
    #[case("rakenne")] // Finnish
    #[case("структура")] // Russian
    #[case("ストラクチャ")] // Japanese
    #[case("结构体")] // Chinese
    #[case("구조체")] // Korean
    #[case("mifumo")] // Swahili
    fn test_struct_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::STRUCT_KW));
    }

    #[rstest]
    #[case("super")] // English
    #[case("supérieur")] // French
    #[case("superieur")] // Dutch
    #[case("superior")] // Danish, Norwegian, Swedish, Portuguese, Spanish
    #[case("superiore")] // Italian
    #[case("ylhäältä")] // Finnish
    #[case("супер")] // Russian
    #[case("スーパー")] // Japanese
    #[case("超级")] // Chinese
    #[case("슈퍼")] // Korean
    #[case("juu")] // Swahili
    fn test_super_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::SUPER_KW));
    }

    #[rstest]
    #[case("trait")] // English, French
    #[case("rasgo")] // Spanish
    #[case("eigenschap")] // Dutch
    #[case("egenskap")] // Danish, Norwegian, Swedish
    #[case("tratto")] // Italian
    #[case("característica")] // Portuguese
    #[case("piirre")] // Finnish
    #[case("характеристика")] // Russian
    #[case("トレイト")] // Japanese
    #[case("特征")] // Chinese
    #[case("특성")] // Korean
    #[case("tabia")] // Swahili
    fn test_trait_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::TRAIT_KW));
    }

    #[rstest]
    #[case("type")] // English, Dutch
    #[case("taper")] // French
    #[case("tipo")] // Spanish, Portuguese, Italian
    #[case("typ")] // Swedish, Danish, Norwegian, German
    #[case("tyyppi")] // Finnish
    #[case("тип")] // Russian
    #[case("タイプ")] // Japanese
    #[case("类型")] // Chinese
    #[case("유형")] // Korean
    #[case("aina")] // Swahili
    fn test_type_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::TYPE_KW));
    }

    #[rstest]
    #[case("unsafe")] // English
    #[case("inseguro")] // Spanish, Portuguese
    #[case("insécurisé")] // French
    #[case("unsicher")] // German
    #[case("onveilig")] // Dutch
    #[case("osäker")] // Swedish
    #[case("usikker")] // Danish, Norwegian
    #[case("insicuro")] // Italian
    #[case("epävarma")] // Finnish
    #[case("небезопасный")] // Russian
    #[case("アンセーフ")] // Japanese
    #[case("不安全")] // Chinese
    #[case("불안전")] // Korean
    #[case("haramu")] // Swahili
    fn test_unsafe_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::UNSAFE_KW));
    }

    #[rstest]
    #[case("use")] // English
    #[case("utilizar")] // Spanish
    #[case("utiliser")] // French
    #[case("gebruiken")] // Dutch
    #[case("använda")] // Swedish
    #[case("bruke")] // Norwegian
    #[case("benytte")] // Danish
    #[case("usare")] // Italian
    #[case("usar")] // Portuguese
    #[case("käyttää")] // Finnish
    #[case("использовать")] // Russian
    #[case("使用する")] // Japanese
    #[case("使用")] // Chinese
    #[case("사용")] // Korean
    #[case("tumia")] // Swahili
    fn test_use_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::USE_KW));
    }

    #[rstest]
    #[case("where")] // English
    #[case("dónde")] // Spanish
    #[case("où")] // French
    #[case("waarin")] // Dutch
    #[case("var")] // Swedish
    #[case("hvor")] // Norwegian
    #[case("hvor")] // Danish
    #[case("dove")] // Italian
    #[case("onde")] // Portuguese
    #[case("missä")] // Finnish
    #[case("где")] // Russian
    #[case("どこ")] // Japanese
    #[case("哪里")] // Chinese
    #[case("어디")] // Korean
    #[case("wapi")] // Swahili
    fn test_where_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::WHERE_KW));
    }

    #[rstest]
    #[case("yield")] // English
    #[case("rendement")] // French, Dutch
    #[case("rendimiento")] // Spanish
    #[case("avkastning")] // Swedish, Norwegian
    #[case("afkastning")] // Danish
    #[case("rendimento")] // Italian, Portuguese
    #[case("rendite")] // German
    #[case("tuotto")] // Finnish
    #[case("доход")] // Russian
    #[case("収益")] // Japanese
    #[case("收益")] // Chinese
    #[case("수익")] // Korean
    #[case("kupato")] // Swahili
    fn test_yield_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::YIELD_KW));
    }
}
