use derivative::Derivative;
use derive_builder::Builder;
use derive_more::Display;
use smartstring::alias::String;
use std::fmt::Display;

use semver::Version as Semver;

pub const LEAFC_VERSION: &str = concat!(
    "leafc v",
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("LEAFC_COMMIT_HASH"),
    " ",
    env!("LEAFC_BUILD_DATE"),
    ")",
);

pub const LEAFC_TARGET: &str = env!("LEAFC_TARGET_TRIPLE");

// TODO derive more traits for `Version` enum
#[derive(Debug, Clone, PartialEq, Eq)]
// Todo: implement comparison operators for semver vs commit hash based on
// calculation of temporal distance between the two, additionally implement
// `Ord` trait for `Version` enum
/// The **version** of the compiler. This is either a **semantic version** or a **commit hash**.
/// The **semantic version** is the **version** of the compiler that is **published** to **crates.io**,
/// while the **commit hash** is the **version** of the compiler that is **built from source**.
/// The **semantic version** is used to **determine** the **version** of the compiler that is **installed**.
/// The **commit hash** is used to **determine** the **version** of the compiler that is **built from source**.
///
/// # Examples
///
/// ```rust
/// use leafc_cfg::settings::Version;
///
/// let version = Version::Semver(Semver::new(0, 1, 0));
/// assert_eq!(version.to_string(), "0.1.0");
///
/// let version = Version::CommitHash(CommitHash::new("a1b2c3d4"));
/// assert_eq!(version.to_string(), "commit hash");
/// ```
// ANCHOR: input
// #[salsa::input]
pub enum Version {
    /// The **semantic version** of the compiler (_e.g. `0.19.1`_).
    Semver(Semver),
    /// The **commit hash** of the compiler (_e.g. `a1b2c3d4`_).
    CommitHash(CommitHash),
}
// ANCHOR_END: input

#[derive(Debug, Display, Clone, PartialEq, Eq, Builder, Derivative)]
#[derivative(Default)]
// #[derive(new)]
#[display(fmt = "{hash}")]
// ?] [{timestamp} UTC]
pub struct CommitHash {
    /// The **hash** of the commit (_e.g. `a1b2c3d4`_)
    /// // i want the internal representation to be a `smartstring::alias::String`
    /// // but i want the external representation to be a `&str`
    // #[new(value = "String::new().to_owned().as_str()")]
    hash: String,

    /// The **timestamp** of the commit (_e.g. `2021-01-01 00:00:00`_)
    timestamp: String,
}

// #[salsa::input]
impl CommitHash {
    pub fn new(hash: &str) -> Self {
        Self { hash: hash.to_owned().into(), timestamp: String::new() }
    }
}

// TODO: update this by deriving `Display` and `FromStr` traits
impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Semver(semver) => write!(f, "{semver}"),
            Self::CommitHash(_) => Ok({
                // let str = String::from("commit hash").to_owned().as_str();

                // write!(f, "{str}")
                let commit_hash = match self {
                    Self::CommitHash(commit_hash) => commit_hash,
                    _ => unreachable!(),
                };
            }),
        }
    }
}

#[cfg(test)]
mod version_test_suite {
    use super::*;
    use pretty_assertions_sorted::assert_eq;

    #[test]
    fn smoke_test_version_api() {
        // SemVer

        // let version = Version::Semver("0.1.0");
        // let version = Version::new("0.1.0");
        // let version = VersionBuilder::new().semver("0.1.0").build();
        // let version = VersionBuilder::new().major(0).minor(1).patch(0).build();
        // assert_eq!(version.to_string(), "0.1.0");

        // todo: create a Metadata crate
        // metadata contains the following:
        // - version
        // - commit hash
        // - timestamp
        // - build date
        // - build time
        // - build duration
        // - build environment
        // - build target
        // - build profile
        // - build features
        // - build dependencies

        // use rstest for testing here

        // CommitHash

        let commit_hash = CommitHash::new("a1b2c3d4");
        let commit_hash = CommitHashBuilder::default()
            .hash("a1b2c3d4".into())
            .timestamp("2021-01-01 00:00:00".into())
            .build()
            .unwrap();

        assert_eq!(commit_hash.to_string(), "a1b2c3d4");

        // let version = Version::new("a1b2c3d4");
        // assert_eq!(version.to_string(), "a1b2c3d4");

        // #[test]
        // fn test_version() {
        //     let version = Version::Semver(Semver::new(0, 1, 0));
        //     assert_eq!(version.to_string(), "0.1.0");
        // }

        // let commit_hash = CommitHash::new("a1b2c3d4");
        // assert_eq!(commit_hash.hash, "a1b2c3d4");
    }
}
