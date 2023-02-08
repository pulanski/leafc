#![allow(dead_code, unused)]
mod history;
mod prompt;
mod syntax_highlighter;

use std::{fs::File, process::ExitCode};

use derivative::Derivative;
use derive_builder::Builder;
use derive_new::new;
use getset::{CopyGetters, Getters, MutGetters, Setters};
use leafc_cfg::settings::{
    repl::{
        ReplSettings, ASM_EXTENSION, AST_EXTENSION, DEFAULT_HISTORY_FILE, DEFAULT_HISTORY_SIZE,
        LLVM_IR_EXTENSION, TOK_EXTENSION,
    },
    EmitKind,
};
use miette::{IntoDiagnostic, Result};
use reedline::{
    ExampleHighlighter, FileBackedHistory, Highlighter, Reedline, Signal, SqliteBackedHistory,
};
use smartstring::alias::String;

use leafc_cli::LeafcCli;
use leafc_driver::LeafcDriver;
use leafc_errors::repl::ReplError;
use smol_str::SmolStr;
use strum::IntoEnumIterator;

use crate::prompt::LeafcPrompt;

#[derive(Debug, new, MutGetters, Getters, Setters)] // getters, setters, and builders
pub struct LeafcRepl {
    /// The repl's **settings**.
    #[new(default)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    settings: ReplSettings,
}

impl LeafcRepl {
    /// Runs the repl.
    fn run(&mut self) -> Result<ExitCode> {
        let mut line_editor = self.setup_line_editor()?;
        let mut prompt = self.setup_prompt()?;

        let mut driver = LeafcDriver::new();

        // TODO: string to hold the current source text,
        // needed for multi-line input

        loop {
            let sig = line_editor.read_line(&prompt);
            match sig {
                Ok(Signal::Success(buffer)) => {
                    // convert the buffer to an optimized string
                    let mut buffer: String = buffer.into();

                    // check if the line is empty and the user entered backspace
                    // if so, flash the prompt gray and continue
                    // if buffer.is_empty() {
                    //     line_editor.flash_prompt_gray();
                    //     continue;
                    // }

                    // if the line is empty and the user entered enter, flash the prompt red
                    // and continue
                    // if buffer.is_empty() {
                    //     line_editor.flash_prompt_red(); // pseudo code
                    //     continue;
                    // }

                    // update the repl's settings if the user entered a setting
                    let (updated, updated_config) =
                        self.settings_mut().update_from_source_text(&mut buffer)?;

                    // if the settings were updated, apply them to the driver
                    if updated {
                        driver.apply_repl_settings(&updated_config);
                    }

                    // if empty, no need to compile
                    if buffer.is_empty() {
                        continue;
                    }

                    // if the line ends in `{`, add a newline and a `}` to the buffer

                    // Run a compilation pass on the line
                    driver.compile(&buffer, true)?;

                    prompt.increment_line_count();
                }

                Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                    println!("\nThanks for using Leafc!");
                    break;
                }

                x => {
                    println!("Event: {x:?}");
                }
            }
        }

        Ok(ExitCode::SUCCESS)
    }

    fn setup_history(&mut self) -> Result<Box<FileBackedHistory>> {
        let history_file =
            FileBackedHistory::with_file(DEFAULT_HISTORY_SIZE, DEFAULT_HISTORY_FILE.into())
                .into_diagnostic()
                .map_err(|e| {
                    ReplError::HistoryFileOpen(
                        "{e}: Could not open history file {DEFAULT_HISTORY_FILE}".into(),
                    )
                })?;

        Ok(Box::new(history_file))
    }

    fn setup_syntax_highlighter(&mut self) -> Result<Box<impl Highlighter>> {
        let syntax_highlighter = ExampleHighlighter::default();

        Ok(Box::new(syntax_highlighter))
    }

    fn setup_prompt(&self) -> Result<LeafcPrompt> {
        Ok(LeafcPrompt::default())
    }

    fn setup_line_editor(&mut self) -> Result<Reedline> {
        // Set the history
        let history = self.setup_history()?;

        // Set the syntax highlighter
        let syntax_highlighter = self.setup_syntax_highlighter()?;

        // TODO: make this configurable

        let mut line_editor =
            Reedline::create().with_history(history).with_highlighter(syntax_highlighter);

        Ok(line_editor)
    }
}

/// Entrypoint for the repl. Sets up the repl configuration and context.
pub fn entry(cli: &LeafcCli) -> Result<ExitCode> {
    leafc_log::logo();
    leafc_utils::vertical_padding(1);

    // TODO: refactor this to be LeafcRepl::run(settings: ReplSettings);
    let mut repl = LeafcRepl::new();
    repl.run()
}
