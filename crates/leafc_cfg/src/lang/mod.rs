use std::str::FromStr;

use derivative::Derivative;
use derive_builder::Builder;
use derive_more::Display;
use getset::{Getters, MutGetters, Setters};
use rust_i18n::t;
use smartstring::alias::String;
use strum::EnumVariantNames;
use strum_macros::EnumString;
use sys_locale::get_locale;

/// ## Languages
///
/// Used when emitting diagnostics and other messages (e.g. error messages) to the user
/// concerning language configuration (i.e. when adding another language to the project,
/// or when changing the current language).
///
/// # Example
///
/// ```text
/// current_lang: English => ENGLISH_LANGUAGE_REF: English
/// current_lang: French  => ENGLISH_LANGUAGE_REF: Anglais
///         ^                              ^
/// language of execution          refrence to English
/// ```

//
// pub static ENGLISH_LANGUAGE_REF: Lazy<String> = Lazy::new(|| t!("leafc_cfg.lang.en"));

// /// Represents the **Spanish** language given the current [**language of execution**][Language].
// pub static SPANISH_LANGUAGE_REF: Lazy<String> = Lazy::new(|| t!("leafc_cfg.lang.es"));

// /// Represents the **French** language given the current [**language of execution**][Language].
// pub static FRENCH_LANGUAGE_REF: Lazy<String> = Lazy::new(|| t!("leafc_cfg.lang.fr"));

// /// Represents the **German** language given the current [**language of execution**][Language].
// pub static GERMAN_LANGUAGE_REF: Lazy<String> = Lazy::new(|| t!("leafc_cfg.lang.de"));

// /// Represents the **Italian** language given the current [**language of execution**][Language].
// pub static ITALIAN_LANGUAGE_REF: Lazy<String> = Lazy::new(|| t!("leafc_cfg.lang.it"));

// /// Represents the **Japanese** language given the current [**language of execution**][Language].
// pub static JAPANESE_LANGUAGE_REF: Lazy<String> = Lazy::new(|| t!("leafc_cfg.lang.ja"));

// /// Represents the **Korean** language given the current [**language of execution**][Language].
// pub static KOREAN_LANGUAGE_REF: Lazy<String> = Lazy::new(|| t!("leafc_cfg.lang.ko"));

// /// Represents the **Chinese** language given the current [**language of execution**][Language].
// pub static CHINESE_LANGUAGE_REF: Lazy<String> = Lazy::new(|| t!("leafc_cfg.lang.zh"));

// /// Represents the **Portuguese** language given the current [**language of execution**][Language].
// pub static PORTUGUESE_LANGUAGE_REF: Lazy<String> = Lazy::new(|| t!("leafc_cfg.lang.pt"));

// pub type LangRefs = ENGLISH_LANGUAGE_REF;

// pub type ENGLISH_LANGUAGE = t!("leafc_cfg.lang.en");

/// Retrieves the **default language** that the compiler should use for both
/// compilation and execution as well as for emitting diagnostics and other
/// messages (e.g. error messages) to the user. It is determined from the
/// user's **system locale** and **language preferences** (e.g. the user's
/// **language settings** in their operating system).
///
/// This data is not used for anything other than to determine the default
/// language that the compiler should use.
///
/// # Examples
///
/// ```rust
/// use leafc_cfg::lang::{default_language, Language};
///
/// // The default language is retrieved (will vary depending on the user's
/// // system locale and language preferences).
/// let language: Language = default_language();
pub fn default_language() -> Language {
    let lang_code = get_locale().unwrap_or_default();
    Language::from_str(&lang_code).unwrap_or(Language::English)
}

// TODO: when instantiating the LanguageConfiguration, check and see if it is set
// within the config file. If it is, then use the `Builder` pattern to instantiate
// the LanguageConfiguration. If it is not, then use the `new` function via Default.
pub fn init() -> LanguageConfiguration {
    LanguageConfiguration::new()
}

/// Stores the **language configuration** for the project. This includes the
/// various **languages** that the project is able to support when compiling
/// from source code, as well as the **current language** that the project is
/// using for emitting diagnostics and other messages (e.g. error messages) to
/// the user during both compilation and execution.
///
/// # Example
///
/// ```rust
///  use leafc_cfg::lang::{default_language, Language, LanguageConfiguration};
///
/// let lang_cfg = LanguageConfiguration::new();
///
/// // The default language is retrieved.
/// let language: &Language = lang_cfg.current_language();
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Getters, MutGetters, Setters, Derivative, Builder)]
#[derivative(Default(new = "true"))]
pub struct LanguageConfiguration {
    /// The various **languages** that the project is able to support. This
    /// means that the compiler will be able to **compile** the project using
    /// the specified languages as source code.
    #[getset(get_copy = "pub", get_mut, set)]
    #[derivative(Default(value = "vec![default_language()]"))]
    pub supported_languages: Vec<Language>,

    /// The **current language** that the project is using. This means that
    /// the compiler will be use the specified language for emitting
    /// diagnostics and other messages (e.g. error messages) to the user
    /// during both compilation and execution.
    #[getset(get = "pub", get_mut, set)]
    #[derivative(Default(value = "default_language()"))]
    pub current_language: Language,
}

impl LanguageConfiguration {
    /// Adds a **new language** to the **list of languages** that the project is **able to support**
    /// (i.e. the compiler will be able to **compile** the project using the specified language as
    /// source code).
    ///
    /// # Errors
    ///
    /// If the language is already supported by the project, then an error is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use leafc_cfg::lang::LanguageConfiguration;
    ///
    /// let mut lang_cfg = LanguageConfiguration::new();
    ///
    /// // Add a new language to the list of languages that the project is able to support.
    /// lang_cfg.add_language("en").unwrap();
    /// ```
    pub fn add_language(&mut self, lang: &str) -> Result<(), String> {
        let lang = Language::from_str(lang).map_err(|_| format!("invalid language: {lang}"))?;

        // if self.supported_languages.contains(&lang) {
        //     return Err(format!("language already supported: {}", lang));
        // }

        self.supported_languages.push(lang);

        Ok(())
    }
}

/// All possible languages that the compiler is able to support as well as
/// be configured to use.
///
/// **NOTE**: This list is not exhaustive. It is only a list of languages that
/// are currently supported by the compiler and could always be exptended
/// in the future.
#[derive(
    Clone, Copy, Debug, Default, Display, Eq, Hash, PartialEq, EnumString, EnumVariantNames,
)]
pub enum Language {
    /// The **English** language.
    ///
    /// This includes the following **language tags**:
    ///
    /// | Language Tag | Description |
    /// |--------------|-------------|
    /// | `en`         | _English_     |
    /// | `en-AU`      | _English (Australia)_ |
    /// | `en-CA`      | _English (Canada)_ |
    /// | `en-GB`      | _English (United Kingdom)_ |
    /// | `en-IE`      | _English (Ireland)_ |
    /// | `en-IN`      | _English (India)_ |
    /// | `en-NZ`      | _English (New Zealand)_ |
    /// | `en-US`      | _English (United States)_ |
    /// | `en-ZA`      | _English (South Africa)_ |
    #[strum(
        serialize = "en",
        serialize = "en-AU",
        serialize = "en-CA",
        serialize = "en-GB",
        serialize = "en-IE",
        serialize = "en-IN",
        serialize = "en-NZ",
        serialize = "en-US",
        serialize = "en-ZA",
        serialize = "English"
    )]
    #[default]
    // #[display(fmt = "{}", "ENGLISH_LANGUAGE_REF.as_str()")]
    English,

    /// The **Spanish** language.
    ///
    /// This includes the following **language tags**:
    ///
    /// | Language Tag | Description |
    /// |--------------|-------------|
    /// | `es`         | _Spanish_     |
    /// | `es-AR`      | _Spanish (Argentina)_ |
    /// | `es-CL`      | _Spanish (Chile)_ |
    /// | `es-CO`      | _Spanish (Colombia)_ |
    /// | `es-ES`      | _Spanish (Spain)_ |
    /// | `es-MX`      | _Spanish (Mexico)_ |
    /// | `es-US`      | _Spanish (United States)_ |
    #[strum(
        serialize = "es",
        serialize = "es-AR",
        serialize = "es-CL",
        serialize = "es-CO",
        serialize = "es-ES",
        serialize = "es-MX",
        serialize = "es-US",
        serialize = "Spanish"
    )]
    Spanish,

    /// The **French** language.
    French,

    /// The **German** language.
    German,

    /// The **Portuguese** language.
    Portuguese,

    /// The **Italian** language.
    Italian,

    /// The **Dutch** language.
    Dutch,

    /// The **Swedish** language.
    Swedish,

    /// The **Danish** language.
    Danish,

    /// The **Norwegian** language.
    Norwegian,

    /// The **Finnish** language.
    Finnish,

    /// The **Russian** language.
    Russian,

    /// The **Japanese** language.
    Japanese,

    /// The **Chinese** language.
    Chinese,

    /// The **Korean** language.
    Korean,

    /// The **Swahili** language.
    Swahili,
}

/// Represents the **English** language given the current [**language of execution**][Language].
#[allow(dead_code)]
fn english() -> String {
    t!("leafc_cfg.lang.en").into()
}

// #[derive(Debug, PartialEq, EnumString)]
// pub enum LanguageTag {
//     /// The **English** language.

//     English,
// }

// pub struct Lang {
//     /// The **language**.
//     pub language: Language,

//     /// The **BCP 47 Language Tag** for the language.
//     /// This is the **language tag** that is used to identify the language.
//     pub tag: &'static str,

//     /// The **language name**.
//     /// This is the name of the language.
//     pub name: &'static str,
//     // /// The **language name** in the language itself.
// }

#[cfg(test)]
mod language_test_suite {
    use pretty_assertions_sorted::assert_eq;
    use rstest::rstest;
    use rust_i18n::{locale, set_locale};
    use std::str::FromStr;
    use strum::VariantNames;

    use crate::lang::{default_language, english, Language};

    #[test]
    fn smoke_default_language() {
        assert_eq!(default_language(), Language::English);
    }

    #[test]
    fn smoke_locale_change() {
        let lang = default_language();
        assert_eq!(lang, Language::English);

        // Tests that the various references are in the current execution language.
        assert_eq!(english(), "English");

        // switch to a different locale
        set_locale("de");
        assert_eq!(locale(), "de");

        assert_eq!(english(), "Englisch");
    }

    #[rstest]
    #[case::english(Language::English, "en")]
    #[case::english_australia(Language::English, "en-AU")]
    #[case::english_canada(Language::English, "en-CA")]
    #[case::english_united_kingdom(Language::English, "en-GB")]
    #[case::english_ireland(Language::English, "en-IE")]
    #[case::english_india(Language::English, "en-IN")]
    #[case::english_new_zealand(Language::English, "en-NZ")]
    #[case::english_united_states(Language::English, "en-US")]
    #[case::english_south_africa(Language::English, "en-ZA")]
    #[case::english_default(Language::English, "foo bar baz")]
    fn smoke_lang_string(#[case] language: Language, #[case] tag: &str) {
        let language_tag: Language = Language::from_str(tag).unwrap_or_default();

        assert_eq!(language, language_tag);
    }

    #[test]
    fn smoke_enum_variants() {
        assert_eq!(
            Language::VARIANTS,
            vec![
                "English",
                "Spanish",
                "French",
                "German",
                "Portuguese",
                "Italian",
                "Dutch",
                "Swedish",
                "Danish",
                "Norwegian",
                "Finnish",
                "Russian",
                "Japanese",
                "Chinese",
                "Korean",
                "Swahili",
            ]
        );
    }
}
