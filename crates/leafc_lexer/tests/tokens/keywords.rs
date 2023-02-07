mod keyword_test_suite {
    use leafc_lexer::TokenKind;
    use logos::Logos;
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
    #[case("pkg")] // English
    #[case("paquete")] // Spanish
    #[case("paquet")] // French
    #[case("pakket")] // Dutch
    #[case("paket")] // Swedish, Danish, Norwegian
    #[case("pacchetto")] // Italian
    #[case("pacote")] // Portuguese
    #[case("paketti")] // Finnish
    #[case("пакет")] // Russian
    #[case("パッケージ")] // Japanese
    #[case("包")] // Chinese
    #[case("패키지")] // Korean
    #[case("vifurushi")] // Swahili
    fn test_pkg_keyword(#[case] raw_token: &str) {
        let mut token = TokenKind::lexer(raw_token);

        assert_eq!(token.next(), Some(TokenKind::PACKAGE_KW));
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
