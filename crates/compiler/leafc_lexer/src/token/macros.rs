/// ---------------------------------------------------------------------------
/// Documentation-related macros
/// ---------------------------------------------------------------------------

/// Macro used to include the documentation for a **general** token.
macro_rules! GENERAL {
    ($path:literal) => {
        include_str!(concat!(TOKEN_DOCS_RELATIVE_PATH!(), "general/", $path, ".md"))
    };
}

/// Macro used to include the documentation for a **comment** token.
macro_rules! COMMENTS {
    ($path:literal) => {
        include_str!(concat!(TOKEN_DOCS_RELATIVE_PATH!(), "comments/", $path, ".md"))
    };
}

/// Macro used to include the documentation for a **literal** token.
macro_rules! LITERALS {
    ($path:literal) => {
        include_str!(concat!(TOKEN_DOCS_RELATIVE_PATH!(), "literals/", $path, ".md"))
    };
}

/// Macro used to include the documentation for a **mathematical symbol** token.
macro_rules! MATHEMATICAL_SYMBOLS {
    ($path:literal) => {
        include_str!(concat!(TOKEN_DOCS_RELATIVE_PATH!(), "mathematical_symbols/", $path, ".md"))
    };
}

/// Macro used to include the documentation for a **reserved keyword** token.
macro_rules! RESERVED_KEYWORDS {
    ($path:literal) => {
        include_str!(concat!(TOKEN_DOCS_RELATIVE_PATH!(), "keywords/reserved/", $path, ".md"))
    };
}

/// Macro used to include the documentation for a **keyword** token.
macro_rules! KEYWORDS {
    ($path:literal) => {
        include_str!(concat!(TOKEN_DOCS_RELATIVE_PATH!(), "keywords/", $path, ".md"))
    };
}

/// Macro used to include the documentation for a **punctuation** token.
macro_rules! PUNCTUATION {
    ($path:literal) => {
        include_str!(concat!(TOKEN_DOCS_RELATIVE_PATH!(), "punctuation/", $path, ".md"))
    };
}

/// Macro used to get the relative path to the token documentation.
macro_rules! TOKEN_DOCS_RELATIVE_PATH {
    () => {
        "../../../../../docs/src/chapter_1/lexical_structure/tokens/"
    };
}

/// Macro defining the documentation for the `TOKEN.md` file.
macro_rules! TOKEN_README {
    () => {
        include_str!("../../TOKEN.md")
    };
}

/// ---------------------------------------------------------------------------
/// Token-related macros
/// ---------------------------------------------------------------------------

/// Macro used for matching a **generic token** within the context of the
/// `LanguageChecker` (e.g. `TokenKind::IDENTIFIER`). Generic tokens are tokens
/// that are not specific to a particular spoken language.
macro_rules! generic_tokens {
    () => {
        // ----------------------------------------------------------------
        // Generic tokens
        // ----------------------------------------------------------------
        TokenKind::ERROR |
            TokenKind::WHITESPACE |
            TokenKind::IDENTIFIER |
            TokenKind::COMMENT |
            TokenKind::DOC_COMMENT |
            TokenKind::RUNE |
            TokenKind::STRING |
            TokenKind::RAW_STRING |
            TokenKind::INTEGER |
            TokenKind::FLOAT |
            TokenKind::LIFETIME |
            TokenKind::INTEGER_SUP |
            TokenKind::FLOAT_SUP |
            TokenKind::PI |
            TokenKind::EULER |
            TokenKind::PHI |
            TokenKind::TAU |
            TokenKind::CATALAN |
            TokenKind::EULERGAMMA |
            TokenKind::INF |
            TokenKind::NAN |
            TokenKind::DEFINE |
            TokenKind::PLUS |
            TokenKind::MINUS |
            TokenKind::STAR |
            TokenKind::SLASH |
            TokenKind::PERCENT |
            TokenKind::CARET |
            TokenKind::BANG |
            TokenKind::AMPERSAND |
            TokenKind::PIPE |
            TokenKind::DOUBLE_AMPERSAND |
            TokenKind::DOUBLE_PIPE |
            TokenKind::SHL |
            TokenKind::SHR |
            TokenKind::PLUS_EQ |
            TokenKind::MINUS_EQ |
            TokenKind::STAR_EQ |
            TokenKind::SLASH_EQ |
            TokenKind::PERCENT_EQ |
            TokenKind::CARET_EQ |
            TokenKind::AMPERSAND_EQ |
            TokenKind::PIPE_EQ |
            TokenKind::SHL_EQ |
            TokenKind::SHR_EQ |
            TokenKind::EQ |
            TokenKind::EQEQ |
            TokenKind::NE |
            TokenKind::GT |
            TokenKind::LT |
            TokenKind::GE |
            TokenKind::LE |
            TokenKind::AT |
            TokenKind::UNDERSCORE |
            TokenKind::DOT |
            TokenKind::DOTDOT |
            TokenKind::DOTDOTEQ |
            TokenKind::COMMA |
            TokenKind::SEMICOLON |
            TokenKind::COLON |
            TokenKind::PATHSEP |
            TokenKind::RARROW
    }; /* ERROR, WHITESPACE, IDENTIFIER, COMMENT, DOC_COMMENT, RUNE,
        *         STRING, RAW_STRING, INTEGER, FLOAT, LIFETIME, INTEGER_SUP,
        *         FLOAT_SUP, PI, EULER, PHI, TAU, CATALAN, EULERGAMMA,
        *         INF, NAN, DEFINE, PLUS, MINUS, STAR, SLASH, PERCENT, CARET,
        *         BANG, AMPERSAND, PIPE, DOUBLE_AMPERSAND, DOUBLE_PIPE, SHL,
        *         SHR, PLUS_EQ, MINUS_EQ, STAR_EQ, SLASH_EQ, PERCENT_EQ,
        *         CARET_EQ, AMPERSAND_EQ, PIPE_EQ, SHL_EQ, SHR_EQ, EQ, EQEQ,
        *         NE, GT, LT, GE, LE, AT, UNDERSCORE, DOT, DOTDOT, DOTDOTEQ,
        *         COMMA, SEMICOLON, COLON, PATHSEP, RARROW, FATARROW, HASH,
        *         DOLLAR, QMARK, TILDE, L_BRACKET, R_BRACKET, L_PAREN, R_PAREN,
        *         L_BRACE, R_BRACE, L_PAREN_SUPERSCRIPT, R_PAREN_SUPERSCRIPT,
        *         L_ARROW, DOUBLE_STAR */
}

/// Macro used for matching a **language-specific token** within the
/// context of the `LanguageChecker` (e.g. `TokenKind::ABSTRACT_KW`).
macro_rules! language_specific_token {
    ($kind:ident, $($lexeme:literal),* $(,)?) => {
        match self.kind() {
            TokenKind::$kind => match self.lexeme().as_str() {
                $( $lexeme => true, )*
                _ => false,
            },
            _ => false,
        }
    };
}
