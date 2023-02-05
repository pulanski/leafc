use chrono::{DateTime, Local};
use derive_more::{Display, From};

/// Get the current time in the format of `YYYY-MM-DD HH:MM:SS`.
/// // TODO: Make this configurable via a specific time format string.
pub fn now() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

// Add,
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display, From)]
#[display(fmt = "{}", "time.format(format)")]
pub struct Time {
    time: DateTime<Local>,
    format: String,
}

// #[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Add, Display, From)]
// pub enum TimeFormat {
//     // #[display(fmt = "%Y-%m-%d %H:%M:%S")]
//     #[default]
//     Default,
//     Custom(String),
// }

impl Time {
    // pub fn now(format: TimeFormat) -> Self {
    //     Self { time: Local::now(), format }
    // }
    pub fn now(format: String) -> Self {
        Self { time: Local::now(), format }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions_sorted::assert_eq;

    #[test]
    fn time_smoke() {
        let time = Time::now("%Y-%m-%d %H:%M:%S".into());
        assert_eq!(time.to_string(), now());
    }
}

// impl fmt::Display for Time {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.time.format(&self.format))
//     }
// }

// impl Time {
//     pub fn new(format: String) -> Self {
//         Self {
//             time: Local::now(),
//             format,
//         }
//     }
// }
