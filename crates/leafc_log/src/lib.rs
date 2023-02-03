pub mod macros;

use fern::{log_file, Dispatch};
use leafc_cfg::settings::log::LogLevel;
use leafc_errors::log::LogError;
use miette::IntoDiagnostic;

pub fn init(verbosity: LogLevel) -> Result<(), LogError> {
    let log_file_name = "leafc.log";

    // if debug mode, also log to file in cwd

    let log_file_path = dirs_next::home_dir()
        .ok_or(LogError::LogFileOpen(
            format!("Could not find home directory for user: {}", whoami::username()).into(),
        ))?
        .join(log_file_name);

    let mut base_log_config = Dispatch::new();

    base_log_config = match verbosity {
        LogLevel::Trace => base_log_config.level(log::LevelFilter::Trace),
        LogLevel::Debug => base_log_config.level(log::LevelFilter::Debug),
        LogLevel::Info => base_log_config.level(log::LevelFilter::Info),
        LogLevel::Warn => base_log_config.level(log::LevelFilter::Warn),
        LogLevel::Error => base_log_config.level(log::LevelFilter::Error),
        LogLevel::Fatal => base_log_config.level(log::LevelFilter::Error),
    };

    // Separate file config so we can include year, month and day in file logs
    let log_file = Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .chain(log_file(log_file_path.clone()).into_diagnostic().map_err(|_| {
            LogError::LogFileInitialization(log_file_path.to_string_lossy().into())
        })?);

    base_log_config
        .chain(std::io::stdout())
        .chain(log_file)
        .apply()
        .into_diagnostic()
        .map_err(|_| LogError::LogSystemInitialization(verbosity.to_string().into()))?;

    eprintln!("{verbosity:?}");

    Ok(())
}
