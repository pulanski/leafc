use leafc_cfg::lang::LanguageKind;

use crate::token::TokenSet;

/// ## [**`LanguageChecker`**][LanguageChecker]
///
/// A **language checker** is a **container** for a **spoken language** and all
/// of its **legal tokens**.
///
/// The **job** of the **language checker** is to **check** a given set of
/// **tokens** and see if they are all considered to be valid within a given
/// **spoken language**. It additionally will and **report** any **errors**,
/// that is, **invalid** tokens it finds during the `check` operation.
///
/// A token is deemed **invalid** if it is **not** a **valid** token of the
/// given spoken language. The **language checker** is **used** during the phase
/// of **lexical analysis** to **check** the **tokens** of the **input string**.
///
///
/// When an invalid token is found, the checker emits an error to a diagnostic
/// sink and continues checking the rest of the tokens.
///
/// # Example:
///
/// ```rust
/// // TODO: refactor example to use rayon and interned strings, and
///
/// // new calls builder internally
/// use leafc_lexer::token::LanguageChecker;
///
/// let lang_checker = LanguageChecker::builder()
///     .lang(LanguageKind::English)
///     .tokens(vec!["false".into(), "true".into(), "Faux".into(), "falsch".into()])
///     .build();
///
/// assert_eq!(lang_checker.lang(), LanguageKind::English);
///
/// let valid = lang_checker.check_tokens();
/// assert_eq!(valid, false);
///
/// let lang_checker = LanguageChecker::new(English, vec!["false", "true", "Faux", "falsch"]);
/// ```
pub struct LanguageChecker {
    pub lang:         LanguageKind,
    pub tokens:       TokenSet,
    pub valid_tokens: TokenSet,
}

impl LanguageChecker {
    // pub fn new(lang: LanguageKind, tokens: TokenSet) -> Self {
    //     debug_assert!(lang.is_spoken()); // don't want the language to be `any`
    //     Self { lang, tokens }
    // }

    pub fn lang(&self) -> LanguageKind {
        self.lang
    }

    pub fn tokens(&self) -> &TokenSet {
        &self.tokens
    }

    // pub fn check_tokens(&self, tokens: &[Token]) -> VecDeque<Token> {
    //     let mut tokens = tokens.iter().copied().collect::<VecDeque<_>>();
    //     // let mut tokens = tokens.par_iter().copied().collect::<VecDeque<_>>();
    //     let mut i = 0;
    //     while i < tokens.len() {
    //         let token = tokens[i];
    //         if !self.tokens().contains(&token) {
    //             tokens.remove(i);
    //         } else {
    //             i += 1;
    //         }
    //     }
    //     tokens
    // }

    // pub fn check_token(&self, token: &Token) -> bool {
    //     self.tokens().contains(token)
    // }
}
