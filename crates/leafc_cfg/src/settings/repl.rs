use derivative::Derivative;
use derive_builder::Builder;
use getset::{CopyGetters, Getters, MutGetters, Setters};
use leafc_errors::repl::ReplError;
use miette::{IntoDiagnostic, Result};
use smartstring::alias::String;

use crate::cli::{CommandLineConfig, CommandLineConfigBuilder};

use super::emit::{EmitKind, EmitKinds};

pub const DEFAULT_HISTORY_FILE: &str = "~/.leafc_history";
pub const DEFAULT_LOG_FILE: &str = "~/.leafc/repl.log";
pub const DEFAULT_HISTORY_SIZE: usize = 100;

pub const TOK_EXTENSION: &str = ".tok";
pub const AST_EXTENSION: &str = ".ast";
pub const HIR_EXTENSION: &str = ".hir";
pub const MIR_EXTENSION: &str = ".mir";
pub const LLVM_IR_EXTENSION: &str = ".ll";
pub const ASM_EXTENSION: &str = ".asm";

#[derive(Debug, Derivative, PartialEq, Eq, Builder, Getters, MutGetters, CopyGetters, Setters)]
#[derivative(Default(new = "true"))]
pub struct ReplSettings {
    /// The **kinds** of output to emit from the compiler (e.g. the `AST`, `LLVM IR`, etc.).
    /// defaults to `vec![]`
    #[derivative(Default(value = "vec![]"))]
    #[builder(default = "vec![]")]
    #[getset(set = "pub")]
    pub emit_kinds: Vec<EmitKind>,

    /// The **history file** to use for the repl.
    /// defaults to `~/.leafc/history`
    #[derivative(Default(value = "DEFAULT_HISTORY_FILE.into()"))]
    #[builder(setter(into), default = "DEFAULT_HISTORY_FILE.into()")]
    #[getset(get = "pub")]
    repl_history_file: smartstring::alias::String,

    /// The **log file** to use for the repl.
    /// defaults to `~/.leafc/repl.log`
    #[builder(setter(into), default = "DEFAULT_LOG_FILE.into()")]
    #[derivative(Default(value = "DEFAULT_LOG_FILE.into()"))]
    #[getset(get = "pub")]
    repl_log_file: smartstring::alias::String,

    /// The **theme** to use for the repl.
    /// defaults to `Theme::Default`
    #[builder(default = "ReplTheme::DarkPlus")]
    #[derivative(Default(value = "ReplTheme::DarkPlus"))]
    #[getset(set = "pub")]
    theme: ReplTheme,

    /// Whether or not to use **syntax highlighting** in the repl.
    /// defaults to `true`
    #[builder(default = "true")]
    #[derivative(Default(value = "true"))]
    #[getset(get_copy = "pub")]
    use_syntax_highlighting: bool,
}

impl ReplSettings {
    /// Returns the **kinds** of output to emit from the compiler (e.g. the `AST`, `LLVM IR`, etc.).
    pub fn emit_kinds(&self) -> EmitKinds {
        self.emit_kinds.clone()
    }

    /// Returns the **theme** to use for the repl.
    pub fn theme(&self) -> ReplTheme {
        self.theme
    }

    pub fn set_history_file(&mut self, history_file: impl Into<String>) {
        self.repl_history_file = history_file.into();
    }

    pub fn set_log_file(&mut self, log_file: impl Into<String>) {
        self.repl_log_file = log_file.into();
    }

    pub fn add_emit_kind(&mut self, emit_kind: EmitKind) {
        self.emit_kinds.push(emit_kind);
    }

    pub fn remove_emit_kind(&mut self, emit_kind: EmitKind) {
        self.emit_kinds.retain(|ek| ek != &emit_kind);
    }

    pub fn update_from_source_text(
        &mut self,
        source_text: &mut String,
    ) -> Result<(bool, CommandLineConfig)> {
        let mut updated = false;
        let source_tmp = source_text.clone();
        let mut lines = source_tmp.lines();

        if let Some(line) = lines.next() {
            let mut emit_kinds = vec![];

            // check if the line contains an emit kind (e.g. .tok, .ast, .hir, .mir, .ll, .asm)
            // if so, remove it from the line and add it to the settings
            for word in line.split_whitespace() {
                if word.starts_with("//") {
                    continue;
                }

                if !updated
                    && (word.contains(TOK_EXTENSION)
                        || word.contains(AST_EXTENSION)
                        || word.contains(HIR_EXTENSION)
                        || word.contains(MIR_EXTENSION)
                        || word.contains(LLVM_IR_EXTENSION)
                        || word.contains(ASM_EXTENSION))
                {
                    updated = true;
                }

                match word {
                    TOK_EXTENSION => {
                        emit_kinds.push(EmitKind::TokenStream);
                        leafc_utils::string::remove_substr(source_text, word);
                    }
                    AST_EXTENSION => {
                        emit_kinds.push(EmitKind::Ast);
                        leafc_utils::string::remove_substr(source_text, word);
                    }
                    // "hir" => emit_kinds.push(EmitKind::Hir),
                    // "mir" => emit_kinds.push(EmitKind::Mir),
                    LLVM_IR_EXTENSION => {
                        emit_kinds.push(EmitKind::LlvmIr);
                        // leafc_log::info!("added emit kind: LlvmIr");
                        leafc_utils::string::remove_substr(source_text, word);
                    }
                    ASM_EXTENSION => {
                        emit_kinds.push(EmitKind::Asm);
                        // leafc_log::info!("added emit kind: Asm");
                        leafc_utils::string::remove_substr(source_text, word);
                    }
                    _ => {}
                }
            }

            for emit_kind in emit_kinds {
                if self.emit_kinds.contains(&emit_kind) {
                    self.remove_emit_kind(emit_kind);
                } else {
                    self.add_emit_kind(emit_kind);
                }
            }
        }

        // use builder pattern to create a new CommandLineConfig
        let config = CommandLineConfigBuilder::default()
            .emit_kinds(self.emit_kinds())
            .build()
            .into_diagnostic()
            .map_err(|e| {
                ReplError::InvalidSettingsUpdate(format!("Couldn't update settings: {e}").into())
            })?;

        Ok((updated, config))
    }
}

/// The **theme** to use for the repl.
/// defaults to `Theme::DarkPlus`
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReplTheme {
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

        assert_eq!(settings.repl_log_file(), "~/.leafc/repl.log");
        assert_eq!(settings.repl_history_file(), "~/.leafc_history");
        assert_eq!(settings.emit_kinds(), vec![]);
        assert_eq!(settings.theme(), ReplTheme::DarkPlus);

        settings.set_emit_kinds(vec![EmitKind::Ast]);
        settings.set_history_file("~/.leafc_history2");
        settings.set_log_file("~/.leafc/repl2.log");
        settings.set_theme(ReplTheme::DarkPlus);

        assert_eq!(settings.emit_kinds(), vec![EmitKind::Ast]);
        assert_eq!(settings.repl_history_file(), "~/.leafc_history2");
        assert_eq!(settings.repl_log_file(), "~/.leafc/repl2.log");
        assert_eq!(settings.theme(), ReplTheme::DarkPlus);

        let mut settings = ReplSettingsBuilder::default()
            .emit_kinds(vec![EmitKind::Ast])
            .repl_history_file("~/.leafc_history")
            .build()
            .unwrap();

        assert_eq!(settings.emit_kinds(), vec![EmitKind::Ast]);
        assert_eq!(settings.repl_history_file(), "~/.leafc_history");
        assert_eq!(settings.repl_log_file(), "~/.leafc/repl.log");
        assert_eq!(settings.theme(), ReplTheme::DarkPlus);

        settings.set_emit_kinds(vec![EmitKind::Ast]);
        settings.set_history_file("~/.leafc_history2");
        settings.set_log_file("~/.leafc/repl2.log");
        settings.set_theme(ReplTheme::DarkPlus);

        assert_eq!(settings.emit_kinds(), vec![EmitKind::Ast]);
        assert_eq!(settings.repl_history_file(), "~/.leafc_history2");
        assert_eq!(settings.repl_log_file(), "~/.leafc/repl2.log");
        assert_eq!(settings.theme(), ReplTheme::DarkPlus);
    }
}
