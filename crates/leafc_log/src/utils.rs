use leafc_cfg::settings::{meta::version::LEAFC_VERSION, LogLevel};
use log::log;

/// Log the **version** of the **leafc** compiler. This
/// includes the **semantic version** of the compiler, the
/// **commit hash** of the compiler, and the **build date**
/// of the compiler (e.g. `leafc v0.19.1 (a1b2c3d4 2021-01-01)
/// target: x86_64-unknown-linux-gnu`).
pub fn version() {
    log!(LogLevel::Info.into(), "{}", LEAFC_VERSION);
}
