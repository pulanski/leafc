use std::process::ExitCode;

use clap::Parser;
use miette::Result;

use leafc_cli::LeafcCli;

pub struct LeafcEntry;

impl LeafcEntry {
    /// Runs the **top level entry point** for **leafc** and returns an exit
    /// code indicating whether the operation was successful.
    ///
    /// This function is the primary entry point for the compiler. It is
    /// responsible for parsing the command line arguments, and then starting
    /// the appropriate pipeline to perform the compilation based on whether the
    /// compiler is running in **repl mode** or not.
    ///
    /// Responsible for setting up the compiler's logging
    /// system, and for handling the output of the compiler.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use entry::LeafcEntry;
    ///
    /// LeafcEntry::run();
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if the driver or repl fails to run
    /// fails to run. This could be due to a number of reasons, including:
    ///
    /// - The driver failed to **parse the command line arguments**.
    /// - The driver failed to **parse the input file**.
    /// - The driver failed to **perform type checking**, or **perform
    ///  name resolution**.
    /// - The driver failed to **generate code.**
    /// - The driver failed to **emit the output**.
    ///
    /// In general, the various subsystems within the compiler will return
    /// errors if they fail to perform their tasks. The driver will then
    /// propagate these errors up to the caller via the `Result` type containing
    /// the corresponding `ExitCode` indicating the reason for the failure for
    /// the particular error.
    pub fn run() -> Result<ExitCode> {
        // TODO: this may be moved to the driver
        // initialize the compiler's settings
        // let settings = LeafcSettings::new(&cli); // this api or
        // let settings = leafc_cfg::init_settings();
        // this in turn calls:

        // mutate the settings based on config files
        // leafc_cfg::update_settings_via_config_files(&mut settings);

        // parse the command line arguments
        let cli = LeafcCli::parse();

        // mutate the settings based on the command line arguments

        // leafc_cfg::update_settings_via_cli(&mut settings, &cli);

        // initialize the logging system
        leafc_log::init(cli.verbosity)?;

        // run the driver or repl as appropriate
        if cli.sources().is_empty() {
            leafc_repl::entry(&cli)?;
        } else {
            leafc_driver::batch_run(&cli)?;
        }

        Ok(ExitCode::SUCCESS)
    }
}
