use leafc_cfg::settings::{
    meta::{logo::LOGO, version::LEAFC_VERSION},
    LogLevel,
};
use log::log;
use owo_colors::OwoColorize;

pub fn compiling_file(file: &str) {
    log!(
        LogLevel::Info.into(),
        "{} {} {}",
        "Compiling".bright_green().bold(),
        file.bright_green().bold(),
        "...".bright_green().bold()
    );
}

/// Log the **version** of the **leafc** compiler. This
/// includes the **semantic version** of the compiler, the
/// **commit hash** of the compiler, and the **build date**
/// of the compiler (e.g. `leafc v0.19.1 (a1b2c3d4 2021-01-01)
/// target: x86_64-unknown-linux-gnu`).
pub fn version() {
    log!(LogLevel::Info.into(), "{}", LEAFC_VERSION);
}

/// Log the **logo** of the **leafc** compiler. This is the
/// **logo** that is **displayed** when the **compiler** is
/// **run** in **REPL** mode or when the **compiler** is
/// **run** with the `--interactive` flag during **batch**
/// mode.
pub fn logo() {
    println!("{}", LOGO());
    // log!(LogLevel::Info.into(), "{}", LOGO());
}
