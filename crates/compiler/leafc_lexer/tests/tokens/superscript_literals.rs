mod superscript_literals_test_suite {
    use leafc_lexer::TokenKind;
    use logos::Logos;
    use pretty_assertions_sorted::assert_eq;
    use rstest::rstest;

    // Test all superscript literals (tokens used in superscript expressions)
    // e.g. 2²

    #[rstest]
    #[case("²")]
    #[case("⁻²⁵")]
    #[case("⁺²³⁴")]
    #[case("⁻⁰ᵇ¹⁰")]
    #[case("³²⁵⁶⁷⁸")]
    #[case("⁻ᵒ⁰²⁴⁴")]
    #[case("⁺ᵒ⁰²³⁴²³⁴")]
    #[case("⁺⁰ˣ⁵⁶⁷⁸⁹ᴮᶜᴰᴱ")]
    #[case("⁻⁰ˣ⁰¹²³⁴ᴮᴰᴱᶜ")]
    #[case("⁺⁰ᵇ⁰¹⁰⁰¹⁰¹⁰¹¹")]
    fn test_superscript_integer(#[case] input: &str) {
        let mut lexer = TokenKind::lexer(input);
        let token = lexer.next();
        assert_eq!(token, Some(TokenKind::INTEGER_SUP));
        assert_eq!(lexer.slice(), input);
    }

    #[rstest]
    #[case("¹·⁰¹²³⁴⁵⁶")]
    #[case("⁺⁰ᵇ¹⁰¹·⁰¹")]
    #[case("⁻⁰ᵇ⁰¹¹·⁰¹")]
    #[case("⁺⁰·⁰¹²³⁴⁵⁶")]
    #[case("⁻⁰·⁰¹²³⁴⁵⁶")]
    #[case("²³¹·⁰¹²³⁴⁵⁴")]
    #[case("⁺⁰⁰⁰¹¹²³⁴·⁰²³⁴¹")]
    #[case("⁻⁰⁰⁰¹²³⁴¹·⁰²³⁴¹")]
    #[case("⁺⁰ˣ⁰¹¹⁴⁵⁶⁷⁸⁹ᴬᴮᶜᴰᴱ²³⁴·⁰²³⁴¹")]
    #[case("⁻⁰ˣ⁰¹²³⁴¹·⁰²⁴⁵⁶⁷⁸⁹ᴬᴮᶜᴰᴱ³⁴¹")]
    fn test_superscript_float(#[case] input: &str) {
        let mut lexer = TokenKind::lexer(input);
        let token = lexer.next();
        assert_eq!(token, Some(TokenKind::FLOAT_SUP));
        assert_eq!(lexer.slice(), input);
    }
}
