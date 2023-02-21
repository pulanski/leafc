[![CI][ci_badge]][github]

<!-- [![Security Audit][security_audit_badge]][github] -->
<!-- [![Coverage][coverage_badge]][github] -->
<!-- [![LoC][loc_badge]][github]
[![Docs.rs][docs_badge]][docs.rs]
[![Crates.io][crates_badge]][crates.io] -->

<!-- [![Security Audit][2]][0]
[![Coverage][3]][4]
[![LoC][5]][0]
[![Docs.rs][6]][7]
[![Crates.io][8]][9] -->

# [`leafc_intern`][docs.rs]

This crate defines the functionality within the [`leafc`][leaf_reference] compiler that is
responsible for **interning strings** as well as **generic data structures**
that are used throughout the compiler. This is done to avoid unnecessary
allocations and to reduce the amount of time spent on memory management tasks
as compilers typically end up creating **LOTS** of data structures. The ideas
utilized here are heavily inspired by the detailed work done in
[`rustc`][memory_management_in_compiler_design].

### Note

This crate can be used in both **no_std** and **std** environments. Additionally, the crate can be used in both **single-threaded** and **multi-threaded** environments via the use of [**feature gates**](#features).

## Examples

```rust
use buffer::pool::manager::BufferPoolManager;
// use common::config::BufferPoolConfig;
// use common::config::PageId;

// TODO: create an overarching example that shows how to use the buffer pool
// manager and replacers together.
```

## Features

To avoid compiling unused dependencies, the **rustub_buffer** crate gates
certain features. With the exception of `buffer_pool`, `replacer`, and
`lru_k`, all features within this crate are **disabled by default**. This means you can use
the implementation of the buffer pool manager from this crate and define your own
novel replacement policy if you wish, without having to compile the various replacement policies
that are provided.

| Feature       | Description                                                                                                        | Default      |
| ------------- | ------------------------------------------------------------------------------------------------------------------ | ------------ |
| `buffer_pool` | A **[buffer pool manager]** ([_read more here_](https://web.stanford.edu/class/cs346/2015/notes/Lecture_One.pdf)). | _`enabled`_  |
| `serde`       | **[Serialization/Deserialization]** support.                                                                       | _`disabled`_ |

Disabled features can be selectively enabled in `Cargo.toml`:

```toml
[dependencies]
buffer = { version = "0.1.0", features = ["lru", "serde"] }
```

Conversely, if you only want to a select set of features (\_e.g. just use the
replacement policies), you can disable the default features:

```toml
[dependencies]
buffer = { version = "0.1.0", default-features = false }
```

[docs.rs]: https://docs.rs/leafc_intern
[ci_badge]: https://github.com/pulanski/leafc/workflows/CI/badge.svg
[github]: https://github.com/pulanski/leafc
[leaf_reference]: https://leaf.dev/reference
[memory_management_in_compiler_design]: https://rustc-dev-guide.rust-lang.org/memory.html?highlight=interning#arenas-and--interning
[buffer pool manager]: pool::manager::BufferPoolManager
[replacement policy]: replacer::Replacer
[lru]: replacer::lru::LruReplacer
[lru-k]: replacer::lru_k::LruKReplacer
[serialization/deserialization]: https://serde.rs

<!-- [1]: https://github.com/Kixiron/lasso/workflows/CI/badge.svg
[2]: https://github.com/Kixiron/lasso/workflows/Security%20Audit/badge.svg
[3]: https://coveralls.io/repos/github/Kixiron/lasso/badge.svg?branch=master
[4]: https://coveralls.io/github/Kixiron/lasso?branch=master
[5]: https://tokei.rs/b1/github/Kixiron/lasso
[6]: https://docs.rs/lasso/badge.svg
[7]: https://docs.rs/lasso
[8]: https://img.shields.io/crates/v/lasso.svg
[9]: https://crates.io/crates/lasso -->
