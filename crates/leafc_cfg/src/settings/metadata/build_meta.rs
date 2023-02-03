use salsa::interned::InternedId;
use serde::{Deserialize, Serialize};

/// Defines the **build metadata** of the compiler This is used for **incremental compilation**.
///
/// This includes the **build date**, **build time**, **build duration**, **build environment**, **build
/// target**, **build profile**, **build features**, **build dependency graph** (_e.g. crates or language-specific
/// mechanism for dividing code in a more modular structure_), etc.
///
/// Build metadata contains information about the **build environment** (_e.g. the **operating system**,
/// **architecture**, **compiler**, **compiler version**, **compiler flags**, **build profile**, **build
/// features**, **build dependencies**, etc._).
///
/// // NOTE: this is a **work in progress**.
///
/// // TODO: Need to create a dependency graph for this
/// // TODO: Need to create an action graph for this
///
/// This is used for **incremental compilation**.
///
/// // TODO: display, from_str, new, default, get_set, builder, etc.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct Build {
    /// The **Universally Unique Lexicographically Sortable Identifier** (ULID) of the build.
    build_id: BuildId,

    /// The **build date** of the compiler (_e.g. `2021-01-01 00:00:00`_).
    date: String,
}

// pub struct BuildId(u128::limits::MAX);
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BuildId(rusty_ulid::Ulid);

#[cfg(test)]
mod build_test_suite {
    use super::*;
    use pretty_assertions_sorted::assert_eq;

    #[test]
    fn smoke_test_build_api() {
        let ulid = rusty_ulid::Ulid::generate();
        let time = std::time::SystemTime::now();
        let build = Build {
            build_id: BuildId(ulid),
            date: humantime::format_rfc3339_seconds(time).to_string(),
        };

        assert_eq!(build.build_id, BuildId(ulid));
        assert_eq!(build.date, humantime::format_rfc3339_seconds(time).to_string());
    }
}
