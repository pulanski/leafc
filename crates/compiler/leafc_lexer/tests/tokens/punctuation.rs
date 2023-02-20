mod punctuation_test_suite {
    use leafc_lexer::TokenKind;
    use logos::Logos;
    use pretty_assertions_sorted::assert_eq;
    use rstest::rstest;

    // Test all punctuation (tokens used in expressions)

    #[rstest]
    #[case(":=")]
    fn test_define(#[case] input: &str) {
        let mut lexer = TokenKind::lexer(input);
        let token = lexer.next();
        assert_eq!(token, Some(TokenKind::DEFINE));
        assert_eq!(lexer.slice(), input);
    }

    #[rstest]
    #[case("+")]
    fn test_plus(#[case] input: &str) {
        let mut lexer = TokenKind::lexer(input);
        let token = lexer.next();
        assert_eq!(token, Some(TokenKind::PLUS));
        assert_eq!(lexer.slice(), input);
    }

    #[rstest]
    #[case("-")]
    fn test_minus(#[case] input: &str) {
        let mut lexer = TokenKind::lexer(input);
        let token = lexer.next();
        assert_eq!(token, Some(TokenKind::MINUS));
        assert_eq!(lexer.slice(), input);
    }

    #[rstest]
    #[case("*")]
    fn test_star(#[case] input: &str) {
        let mut lexer = TokenKind::lexer(input);
        let token = lexer.next();
        assert_eq!(token, Some(TokenKind::STAR));
        assert_eq!(lexer.slice(), input);
    }

    #[rstest]
    #[case("/")]
    fn test_slash(#[case] input: &str) {
        let mut lexer = TokenKind::lexer(input);
        let token = lexer.next();
        assert_eq!(token, Some(TokenKind::SLASH));
        assert_eq!(lexer.slice(), input);
    }

    #[rstest]
    #[case("(")]
    fn test_left_paren(#[case] input: &str) {
        let mut lexer = TokenKind::lexer(input);
        let token = lexer.next();
        assert_eq!(token, Some(TokenKind::L_PAREN));
        assert_eq!(lexer.slice(), input);
    }
}
