mod identifier_test_suite {
    use leafc_lexer::TokenKind;
    use logos::Logos;
    use pretty_assertions_sorted::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("a")]
    #[case("ä")]
    #[case("本")]
    #[case("🦀")]
    #[case("a_")]
    #[case("foo")]
    #[case("東京")]
    #[case("_identifier")]
    #[case("Москва")]
    #[case("Привет")]
    #[case("你好世界")]
    #[case("foo_bar")]
    #[case("المملكة")]
    fn test_identifier(#[case] input: &str) {
        let mut lexer = TokenKind::lexer(input);
        let token = lexer.next();
        assert_eq!(token, Some(TokenKind::IDENTIFIER));
        assert_eq!(lexer.slice(), input);
    }
}
