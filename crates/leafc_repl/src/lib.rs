use smartstring::alias::String;

use derivative::Derivative;
use derive_builder::Builder;
use derive_new::new;
use getset::{CopyGetters, Getters, MutGetters, Setters};
use leafc_cfg::settings::EmitKind;

pub fn run() {
    let mut repl = LeafcRepl::new();
    repl.run();
}

// use leafc_cli::LeafcCli;

// pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, new)] // getters, setters, and builders
pub struct LeafcRepl {
    /// The repl's settings.
    #[new(default)]
    settings: ReplSettings,
}

impl LeafcRepl {
    /// Runs the repl.
    pub fn run(&mut self) {
        println!("{}", leafc_cfg::settings::meta::logo::LOGO());
        eprintln!("Hello from leafc repl!")
    }
}

pub const DEFAULT_HISTORY_FILE: &str = "~/.leafc_history";
pub const DEFAULT_LOG_FILE: &str = "~/.leafc/repl.log";

/// bark.toml <- config file name
///

#[derive(Debug, Derivative, PartialEq, Eq, Builder, Getters, MutGetters, CopyGetters, Setters)]
#[derivative(Default(new = "true"))]
pub struct ReplSettings {
    /// The **kinds** of output to emit from the compiler (e.g. the `AST`, `LLVM IR`, etc.).
    /// defaults to `vec![]`
    #[derivative(Default(value = "vec![]"))]
    #[builder(default = "vec![]")]
    #[getset(set = "pub")]
    emit_kinds: Vec<EmitKind>,

    /// The **history file** to use for the repl.
    /// defaults to `~/.leafc/history`
    #[derivative(Default(value = "DEFAULT_HISTORY_FILE.into()"))]
    #[builder(setter(into), default = "DEFAULT_HISTORY_FILE.into()")]
    #[getset(get = "pub")]
    history_file: smartstring::alias::String,

    /// The **log file** to use for the repl.
    /// defaults to `~/.leafc/repl.log`
    #[builder(setter(into), default = "DEFAULT_LOG_FILE.into()")]
    #[derivative(Default(value = "DEFAULT_LOG_FILE.into()"))]
    #[getset(get = "pub")]
    log_file: smartstring::alias::String,

    /// The **theme** to use for the repl.
    /// defaults to `Theme::Default`
    #[builder(default = "Theme::DarkPlus")]
    #[derivative(Default(value = "Theme::DarkPlus"))]
    #[getset(set = "pub")]
    theme: Theme,
}

impl ReplSettings {
    /// Returns the **kinds** of output to emit from the compiler (e.g. the `AST`, `LLVM IR`, etc.).
    pub fn emit_kinds(&self) -> Vec<EmitKind> {
        self.emit_kinds.clone()
    }

    /// Returns the **theme** to use for the repl.
    pub fn theme(&self) -> Theme {
        self.theme
    }

    pub fn set_history_file(&mut self, history_file: impl Into<String>) {
        self.history_file = history_file.into();
    }

    pub fn set_log_file(&mut self, log_file: impl Into<String>) {
        self.log_file = log_file.into();
    }
}

/// The **theme** to use for the repl.
/// defaults to `Theme::DarkPlus`
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Theme {
    /// Dark+ theme.
    #[default]
    DarkPlus,

    /// Catppuccin theme.
    Catppuccin,

    /// Dracula theme.
    Dracula,
    // TODO: add more themes
}

// TODO: migrate settings into cfg crate (i.e. leafc_cfg crate)

#[cfg(test)]
mod repl_test_suite {
    use super::*;
    use pretty_assertions_sorted::assert_eq;

    #[test]
    fn repl_settings_smoke() {
        // the new() api is the same as the default() api
        // allows for semantics of creating a new instance of a type
        // with default values this is done when objects are created
        // without parameters on initialization
        //
        // when parameters are desired, the builder api is used
        // or derive_new::new
        let mut settings = ReplSettings::new();

        assert_eq!(settings.log_file(), "~/.leafc/repl.log");
        assert_eq!(settings.history_file(), "~/.leafc_history");
        assert_eq!(settings.emit_kinds(), vec![]);
        assert_eq!(settings.theme(), Theme::DarkPlus);

        settings.set_emit_kinds(vec![EmitKind::Ast]);
        settings.set_history_file("~/.leafc_history2");
        settings.set_log_file("~/.leafc/repl2.log");
        settings.set_theme(Theme::DarkPlus);

        assert_eq!(settings.emit_kinds(), vec![EmitKind::Ast]);
        assert_eq!(settings.history_file(), "~/.leafc_history2");
        assert_eq!(settings.log_file(), "~/.leafc/repl2.log");
        assert_eq!(settings.theme(), Theme::DarkPlus);

        let mut settings = ReplSettingsBuilder::default()
            .emit_kinds(vec![EmitKind::Ast])
            .history_file("~/.leafc_history")
            .build()
            .unwrap();

        assert_eq!(settings.emit_kinds(), vec![EmitKind::Ast]);
        assert_eq!(settings.history_file(), "~/.leafc_history");
        assert_eq!(settings.log_file(), "~/.leafc/repl.log");
        assert_eq!(settings.theme(), Theme::DarkPlus);

        settings.set_emit_kinds(vec![EmitKind::Ast]);
        settings.set_history_file("~/.leafc_history2");
        settings.set_log_file("~/.leafc/repl2.log");
        settings.set_theme(Theme::DarkPlus);

        assert_eq!(settings.emit_kinds(), vec![EmitKind::Ast]);
        assert_eq!(settings.history_file(), "~/.leafc_history2");
        assert_eq!(settings.log_file(), "~/.leafc/repl2.log");
        assert_eq!(settings.theme(), Theme::DarkPlus);
    }
}
