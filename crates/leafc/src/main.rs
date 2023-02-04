use miette::Result;
use std::process::ExitCode;

use leafc_driver::LeafcDriver;

fn main() -> Result<ExitCode> {
    // Run the driver.
    LeafcDriver::run()
}
