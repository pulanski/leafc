use owo_colors::OwoColorize;
use std::{borrow::Cow, env};

use crossterm::style::{Color, Stylize};
use derivative::Derivative;
use getset::{Getters, MutGetters, Setters};
use leafc_utils::time::Time;
use reedline::{
    Prompt, PromptEditMode, PromptHistorySearch, PromptHistorySearchStatus, PromptViMode,
};

/// The default prompt indicator
pub static DEFAULT_PROMPT_INDICATOR: &str = "leafc> ";
pub static DEFAULT_VI_INSERT_PROMPT_INDICATOR: &str = ": ";
pub static DEFAULT_VI_NORMAL_PROMPT_INDICATOR: &str = "〉";
pub static DEFAULT_MULTILINE_INDICATOR: &str = "::: ";

/// The default color for the prompt, indicator, and right prompt
pub static DEFAULT_PROMPT_COLOR: Color = Color::Green;
pub static DEFAULT_PROMPT_MULTILINE_COLOR: nu_ansi_term::Color = nu_ansi_term::Color::LightBlue;
pub static DEFAULT_INDICATOR_COLOR: Color = Color::Green;
pub static DEFAULT_PROMPT_RIGHT_COLOR: Color = Color::AnsiValue(5);

// # derive default, new, getters, setters, and builders
#[derive(Clone, Debug, Getters, MutGetters, Setters)]
// Derivative,
#[getset(get = "pub", get_mut = "pub", set = "pub")]
// #[derivative(Default(new = "true"))]
pub struct LeafcPrompt {
    /// The segment that should be rendered in the **left** prompt.
    pub left_prompt: PromptSegment,
    /// The segment that should be rendered in the **right** prompt.
    pub right_prompt: PromptSegment,

    multiline: bool,
    // /// The current edit mode of the prompt.
    // /// This is used to determine which prompt indicator to use.
    // edit_mode: PromptEditMode,
    /// The **line count** for the current prompt.
    /// This is used to reactively increment the prompt indicator as the user
    /// enters more lines.
    line_count: usize,
}

impl LeafcPrompt {
    /// **Increment** the line count for the current prompt.
    ///
    /// This is used to reactively increment the prompt indicator as the user
    /// enters more lines.
    ///
    /// # Example
    ///
    /// ```
    /// use leafc_repl::prompt::LeafcPrompt;
    ///
    /// let mut prompt = LeafcPrompt::new();
    ///
    /// assert_eq!(prompt.line_count(), 0);
    ///
    /// prompt.increment_line_count();
    ///
    /// assert_eq!(prompt.line_count(), 1);
    /// ```
    pub fn increment_line_count(&mut self) {
        self.line_count += 1;
    }
}

impl Default for LeafcPrompt {
    fn default() -> Self {
        Self {
            left_prompt: PromptSegment::WithLineCount,
            right_prompt: PromptSegment::Empty,
            multiline: false,
            // edit_mode: PromptEditMode::Default,
            line_count: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum PromptSegment {
    /// A basic user-defined prompt (i.e. just text)
    Basic(String),
    /// The path of the current working directory
    WorkingDirectory,
    /// The current date and time
    CurrentDateTime,
    /// An empty prompt segment
    Empty,
    WithLineCount,
}

/// Given a prompt segment, render it to a Cow<str> that we can use to
/// easily implement [`Prompt`]'s `render_prompt_left` and `render_prompt_right`
/// functions.
fn render_prompt_segment(prompt: &PromptSegment, line_count: usize) -> Cow<str> {
    match &prompt {
        PromptSegment::Basic(s) => Cow::Borrowed(s),
        PromptSegment::WorkingDirectory => {
            let prompt =
                env::current_dir().unwrap_or_default().to_str().unwrap_or("no path").to_string();
            Cow::Owned(prompt)
        }
        PromptSegment::CurrentDateTime => {
            // TODO: make the time format an enum
            Cow::Owned(Time::now("%Y-%m-%d %H:%M:%S".into()).to_string())
        }
        PromptSegment::Empty => Cow::Borrowed(""),
        PromptSegment::WithLineCount => {
            let p = format!("{}{}{} ", "[".black(), line_count.cyan(), "]".black());
            Cow::Owned(p)
        }
    }
}

// TODO: implement special prompt (right now it's mostly reedline's default)
impl Prompt for LeafcPrompt {
    fn render_prompt_left(&self) -> Cow<str> {
        render_prompt_segment(&self.left_prompt, self.line_count)
    }

    fn render_prompt_right(&self) -> Cow<str> {
        render_prompt_segment(&self.right_prompt, self.line_count)
    }

    fn render_prompt_indicator(
        &self,
        edit_mode: reedline::PromptEditMode,
    ) -> std::borrow::Cow<str> {
        match edit_mode {
            PromptEditMode::Default | PromptEditMode::Emacs => {
                DEFAULT_PROMPT_INDICATOR.green().to_string().into()
            }
            PromptEditMode::Vi(vi_mode) => match vi_mode {
                PromptViMode::Normal => DEFAULT_VI_NORMAL_PROMPT_INDICATOR.into(),
                PromptViMode::Insert => DEFAULT_VI_INSERT_PROMPT_INDICATOR.into(),
            },
            PromptEditMode::Custom(str) => format!("({str})").into(),
        }
    }

    fn render_prompt_multiline_indicator(&self) -> std::borrow::Cow<str> {
        Cow::Borrowed(DEFAULT_MULTILINE_INDICATOR)
    }

    fn render_prompt_history_search_indicator(
        &self,
        history_search: PromptHistorySearch,
    ) -> Cow<str> {
        let prefix = match history_search.status {
            PromptHistorySearchStatus::Passing => "",
            PromptHistorySearchStatus::Failing => "failing ",
        };
        // NOTE: magic strings, given there is logic on how these compose I am not sure if it
        // is worth extracting in to static constant
        Cow::Owned(format!("({}reverse-search: {}) ", prefix, history_search.term))
    }

    /// Get the default multilince prompt color
    fn get_prompt_multiline_color(&self) -> nu_ansi_term::Color {
        DEFAULT_PROMPT_MULTILINE_COLOR
    }

    /// Whether to render right prompt on the last line
    fn right_prompt_on_last_line(&self) -> bool {
        false
    }
}
