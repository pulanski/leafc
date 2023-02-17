#![allow(non_snake_case)]

use chrono::Utc;
use std::env::consts::{
    ARCH,
    FAMILY,
    OS,
};
use std::{
    error::Error,
    process::Command,
};

fn main() -> Result<(), Box<dyn Error>> {
    LEAFC_COMMIT_HASH()?;
    LEAFC_BUILD_DATE();
    LEAFC_TARGET_TRIPLE();

    Ok(())
}

fn LEAFC_COMMIT_HASH() -> Result<(), Box<dyn Error>> {
    println!(
        "cargo:rustc-env=LEAFC_COMMIT_HASH={}",
        String::from_utf8(
            Command::new("git").args(["rev-parse", "--short", "HEAD"]).output()?.stdout,
        )?
    );

    Ok(())
}

fn LEAFC_TARGET_TRIPLE() {
    println!("cargo:rustc-env=LEAFC_TARGET_TRIPLE={OS}-{FAMILY}-{ARCH}");
}

fn LEAFC_BUILD_DATE() {
    println!("cargo:rustc-env=LEAFC_BUILD_DATE={}", Utc::now().format("%Y-%m-%d"));
}
