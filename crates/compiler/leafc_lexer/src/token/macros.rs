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

// TODO: refactor macros to follow the T style syntax (e.g. T![;] corresponds to
// the semicolon token)
//
// KEYWORDS![break] -> T![break]
// KEYWORDS_DOC![break] -> documentation for the break keyword (shared
// documentation between the reference and the lexer docs)
