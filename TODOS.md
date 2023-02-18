# TODOS

-   [ ] In the future, look into creating a version crate
    -   [ ] Version crate should be able to parse version strings of different formats
        -   [ ] SemVer (https://semver.org/) (e.g. 1.0.0 or 1.0.0-alpha.1 or >=1.0.0 <2.0.4)
        -   [ ] Commit hash (e.g. 1234567890abcdef1234567890abcdef12345678)

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

[ ] - need to change repl config file to use `dirs` crate or something else to find config

[ ] - create a `generate_summary` xtask for generating the summary for the `docs`

[ ] - auto get the user's language using their ip address, however, user's can specify the
language of the compiler in the config file

[ ] - fix repl directory from ~, its getting annoying

[ ] - general goal: remove the coupling of ungrammar from rust-analyzer, want to use ungrammar
but not it's quite coupled with rust-analyzer - currently there is a bug where `syntax_gen` is written as a `test`. As such, when formatting is done,
the generated files are modified, and the tests fail in ci. Instead, migrate the `syntax_gen` to a
`bin` and run it as a `xtask` in ci or something similar

[ ] - get a sense of syntactic structure using `lalrpop` and then use that to guide the
rest of the parsing system
