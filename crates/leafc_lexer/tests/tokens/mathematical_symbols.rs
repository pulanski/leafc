mod mathematical_constants_test_suite {
    use leafc_lexer::TokenKind;
    use logos::Logos;
    use pretty_assertions_sorted::assert_eq;
    use rstest::rstest;

    // Test all mathematical constants

    #[rstest]
    #[case("pi")]
    #[case("π")]
    #[case("𝜋")]
    #[case("𝛑")]
    #[case("𝝅")]
    #[case("𝞹")]
    fn test_pi(#[case] input: &str) {
        let mut lexer = TokenKind::lexer(input);
        let token = lexer.next();
        assert_eq!(token, Some(TokenKind::PI));
        assert_eq!(lexer.slice(), input);
    }

    // #[token("𝑒")]
    // #[token("euler")]
    #[rstest]
    #[case("𝑒")]
    #[case("euler")]
    fn test_euler(#[case] input: &str) {
        let mut lexer = TokenKind::lexer(input);
        let token = lexer.next();
        assert_eq!(token, Some(TokenKind::EULER));
        assert_eq!(lexer.slice(), input);
    }

    #[rstest]
    #[case("phi")]
    #[case("golden")]
    #[case("φ")]
    #[case("𝜙")]
    #[case("𝛗")]
    #[case("𝝓")]
    fn test_phi(#[case] input: &str) {
        let mut lexer = TokenKind::lexer(input);
        let token = lexer.next();
        assert_eq!(token, Some(TokenKind::PHI));
        assert_eq!(lexer.slice(), input);
    }

    #[rstest]
    #[case("tau")]
    #[case("τ")]
    #[case("𝜏")]
    #[case("𝛕")]
    #[case("𝝉")]
    fn test_tau(#[case] input: &str) {
        let mut lexer = TokenKind::lexer(input);
        let token = lexer.next();
        assert_eq!(token, Some(TokenKind::TAU));
        assert_eq!(lexer.slice(), input);
    }

    #[rstest]
    #[case("catalan")]
    #[case("K")]
    #[case("𝑘")]
    fn test_catalan(#[case] input: &str) {
        let mut lexer = TokenKind::lexer(input);
        let token = lexer.next();
        assert_eq!(token, Some(TokenKind::CATALAN));
        assert_eq!(lexer.slice(), input);
    }

    #[rstest]
    #[case("γ")]
    #[case("𝛾")]
    #[case("eulergamma")]
    #[case("eulermascheroni")]
    fn test_eulergamma(#[case] input: &str) {
        let mut lexer = TokenKind::lexer(input);
        let token = lexer.next();
        assert_eq!(token, Some(TokenKind::EULERGAMMA));
        assert_eq!(lexer.slice(), input);
    }

    #[rstest]
    #[case("inf")]
    #[case("∞")]
    #[case("∞16")]
    #[case("∞32")]
    #[case("-Inf")]
    #[case("-INF")]
    #[case("inf16")]
    #[case("+Inf16")]
    #[case("-INF16")]
    #[case("inf32")]
    #[case("+Inf32")]
    #[case("INF32")]
    fn test_inf(#[case] input: &str) {
        let mut lexer = TokenKind::lexer(input);
        let token = lexer.next();
        assert_eq!(token, Some(TokenKind::INF));
        assert_eq!(lexer.slice(), input);
    }

    #[rstest]
    #[case("nan")]
    #[case("-NaN")]
    #[case("-NAN")]
    #[case("nan16")]
    #[case("+NaN16")]
    #[case("-NAN16")]
    #[case("nan32")]
    #[case("+NaN32")]
    #[case("NAN32")]
    fn test_nan(#[case] input: &str) {
        let mut lexer = TokenKind::lexer(input);
        let token = lexer.next();
        assert_eq!(token, Some(TokenKind::NAN));
        assert_eq!(lexer.slice(), input);
    }
}
