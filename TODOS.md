
# TODOS

- [ ] In the future, look into creating a version crate
  - [ ] Version crate should be able to parse version strings of different formats
    - [ ] SemVer (https://semver.org/) (e.g. 1.0.0 or 1.0.0-alpha.1 or >=1.0.0 <2.0.4)
    - [ ] Commit hash (e.g. 1234567890abcdef1234567890abcdef12345678)

```rust
// wrapper around SemVer crate, no need to rewrite the wheel
//
// also create builder API for creating versions
// - Version::new()
// - VersionBuilder::new()
//      .
// Simple API for parsing and displaying versions
// functions:
// - is_semver()
// - is_commit_hash()
// - parse()
// - display()
// - from_str()
// - to_string()
// - to_str()
// - from_semver()
// - from_commit_hash()
// - to_semver()
// - to_commit_hash()
// - default()
// - new()
// - new_semver()
// - new_commit_hash()
// - major()
// - minor()
// - patch()
// - pre_release()
// - build_metadata()
// - hash()
// - set_major()
// - set_minor()
// - set_patch()
// - set_pre_release()
// - set_build_metadata()
// - set_hash()
// - is_semver()
// - is_commit_hash()
// - is_valid()
// - is_valid_semver()
// - is_valid_commit_hash()

// TODO: look into using the SemVer crate
// TODO: derive traits for Version
// TODO: derive Display and FromStr for Version
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Version {
    SemVer(SemVer),
    CommitHash(CommitHash),
}

pub struct SemVer {
    major: u32,
    minor: u32,
    patch: u32,
    pre_release: Option<String>,
    build_metadata: Option<String>,
}

pub struct CommitHash {
    hash: String,
}

```
