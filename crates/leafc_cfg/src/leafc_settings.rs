use crate::{cli::CommandLineConfig, settings::LeafcSettings};
use miette::Result;

// settings can be constructed from config files, cli args, or a combination of both
// in general, the precedence is as follows: cli > config file > default values
// to tackle this, we can have a `LeafcSettings` struct that contains all the settings
// and then we can have a `LeafcSettingsBuilder` struct that contains all the settings

pub fn init(cli_config: &CommandLineConfig) -> Result<LeafcSettings> {
    todo!()
}
