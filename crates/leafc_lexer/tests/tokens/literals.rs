mod literal_test_suite {
    use leafc_lexer::TokenKind;
    use logos::Logos;
    use pretty_assertions_sorted::assert_eq;
    use rstest::rstest;

    // Test all literals (tokens used in literal expressions)

    #[rstest]
    #[case("'a'")]
    #[case("'ä'")]
    #[case("'本'")]
    #[case("'🦀'")]
    #[case("'\\n'")]
    #[case("'\\t'")]
    #[case("'\\0'")]
    #[case("'\\r'")]
    #[case("'\\\"'")]
    #[case("'\\x00'")]
    #[case("'\\u{0}'")]
    #[case("'\\u{00000000}'")]
    fn test_rune_literal(#[case] input: &str) {
        let mut lexer = TokenKind::lexer(input);
        let token = lexer.next();
        assert_eq!(token, Some(TokenKind::RUNE));
        assert_eq!(lexer.slice(), input);
    }

    #[rstest]
    #[case("\"\"")]
    #[case("\"a\"")]
    #[case("\"ä\"")]
    #[case("\"本\"")]
    #[case("\"🦀\"")]
    #[case("\"\\'\"")]
    #[case("\"\\0\"")]
    #[case("\"\\r\"")]
    #[case("\"\\n\"")]
    #[case("\"\\t\"")]
    #[case("\"\\\"\"")]
    #[case("\"日本語\"")]
    #[case("\"\\x00\"")]
    #[case("\"\\u{0}\"")]
    #[case("\"\u{65e5}本\"")]
    #[case("\"\\u{00000000}\"")]
    #[case("\"Hello, world!\n\"")]
    fn test_string_literal(#[case] input: &str) {
        let mut lexer = TokenKind::lexer(input);
        let token = lexer.next();
        assert_eq!(token, Some(TokenKind::STRING));
        assert_eq!(lexer.slice(), input);
    }

    #[rstest]
    #[case("r#\"\"#")]
    #[case("r#\"a\"#")]
    #[case("r#\"ä\"#")]
    #[case("r#\"本\"#")]
    #[case("r#\"🦀\"#")]
    #[case("r#\"\\n\"#")]
    #[case("r#\"\\t\"#")]
    #[case("r#\"\\'\"#")]
    #[case("r#\"\\0\"#")]
    #[case("r#\"\\r\"#")]
    #[case("r#\"\\\"\"#")]
    #[case("r#\"日本語\"#")]
    #[case("r#\"\\x00\"#")]
    #[case("r#\"\\u{0}\"#")]
    #[case("r#\"\u{65e5}本\"#")]
    #[case("r#\"\\u{00000000}\"#")]
    #[case("r#\"Hello, world!\n\"#")]
    fn test_raw_string_literal(#[case] input: &str) {
        let mut lexer = TokenKind::lexer(input);
        let token = lexer.next();
        assert_eq!(token, Some(TokenKind::RAW_STRING));
        assert_eq!(lexer.slice(), input);
    }

    #[rstest]
    #[case("0")]
    #[case("123")]
    #[case("0xff")]
    #[case("0o77")]
    #[case("0usize")]
    #[case("123i32")]
    #[case("123u32")]
    #[case("0b1010")]
    #[case("123_u32")]
    #[case("0xff_u32")]
    #[case("0o77_u32")]
    #[case("0xBadFace")]
    #[case("0xBad_Face")]
    #[case("0xdeadbeef")]
    #[case("0b1010_u32")]
    #[case("0b1__00101_0__1_0101___0_u32")]
    #[case("170141183460469231731687303715884105727")]
    #[case("170_141183_460469_231731_687303_715884_105727")]
    fn test_integer_literal(#[case] input: &str) {
        let mut lexer = TokenKind::lexer(input);
        let token = lexer.next();
        assert_eq!(token, Some(TokenKind::INTEGER));
        assert_eq!(lexer.slice(), input);
    }

    #[rstest]
    #[case("inf")]
    #[case("Inf")]
    #[case("0.")]
    #[case("INF")]
    #[case(".25")]
    #[case("+Inf")]
    #[case("-NaN")]
    #[case("1.0E6")]
    #[case("inf32")]
    #[case("inf16")]
    #[case("NAN32")]
    #[case("1.e+0")]
    #[case("+nan16")]
    #[case("072.40")]
    #[case("0.0f32")]
    #[case("072.40")]
    #[case("2.71828")]
    #[case("0x0.0p0")]
    #[case("123.0f32")]
    #[case("123.0_f32")]
    #[case(".12345E+5")]
    #[case("0.15e+0_2")]
    #[case("6.67428e-11")]
    fn test_float_literal(#[case] input: &str) {
        let mut lexer = TokenKind::lexer(input);
        let token = lexer.next();
        assert_eq!(token, Some(TokenKind::FLOAT));
        assert_eq!(lexer.slice(), input);
    }

    #[rstest]
    #[case("'a")]
    #[case("'🦀")]
    #[case("'foo")]
    #[case("'static")]
    fn test_lifetime(#[case] input: &str) {
        let mut lexer = TokenKind::lexer(input);
        let token = lexer.next();
        assert_eq!(lexer.slice(), input);
        assert_eq!(token, Some(TokenKind::LIFETIME));
    }
}
