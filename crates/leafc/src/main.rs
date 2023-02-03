use miette::Result;
use std::process::ExitCode;

use leafc_driver::LeafcDriver;

#[tokio::main]
async fn main() -> Result<ExitCode> {
    // Run the driver.
    LeafcDriver::run().await
}
