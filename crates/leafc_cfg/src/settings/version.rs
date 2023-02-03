use smartstring::alias::String;
use std::fmt::Display;

use semver::Version as Semver;

// TODO derive display and from_string for this enum
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
pub enum Version {
    /// The **semantic version** of the compiler (_e.g. `0.19.1`_).
    Semver(Semver),
    /// The **commit hash** of the compiler (_e.g. `a1b2c3d4`_).
    CommitHash(CommitHash),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitHash {
    /// the **hash** of the commit (_e.g. `a1b2c3d4`_)
    /// // TODO: have the hash be run through a validation function to ensure that it is a valid hash
    hash: String,

    /// The **timestamp** of the commit (_e.g. `2021-01-01 00:00:00`_)
    timestamp: String,

    /// The **ancestor commits** of the commit (_e.g. `["a1b2c3d4", "e5f6g7h8"]`_)
    /// By default, a commit has one ancestor commit
    ancestor_commits: Vec<String>,
    author: String,
    message: String,
    /// the digital signature of the commit
    /// this is used to verify that the commit was not tampered with
    signature: String,
}

// pub struct CommitHash(String);

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Semver(semver) => write!(f, "{}", semver),
            Self::CommitHash(_) => write!(f, "commit hash"),
        }
    }
}

#[test]
fn test_version() {
    let version = Version::Semver(Semver::new(0, 1, 0));
    assert_eq!(version.to_string(), "0.1.0");
}
