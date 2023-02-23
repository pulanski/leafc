# TODOS

-   [ ] In the future, look into creating a version crate
    -   [ ] Version crate should be able to parse version strings of different
            formats
        -   [ ] SemVer (https://semver.org/) (e.g. 1.0.0 or 1.0.0-alpha.1 or >=1.0.0
                <2.0.4)
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

[ ] - need to change repl config file to use `dirs` crate or something else to
find config

[ ] - create a `generate_summary` xtask for generating the summary for the
`docs`

[ ] - auto get the user's language using their ip address, however, user's can
specify the language of the compiler in the config file

[ ] - fix repl directory from ~, its getting annoying

[ ] - general goal: remove the coupling of ungrammar from rust-analyzer, want to
use ungrammar but not it's quite coupled with rust-analyzer - currently there is
a bug where `syntax_gen` is written as a `test`. As such, when formatting is
done, the generated files are modified, and the tests fail in ci. Instead,
migrate the `syntax_gen` to a `bin` and run it as a `xtask` in ci or something
similar

[ ] - get a sense of syntactic structure using `lalrpop` and then use that to
guide the rest of the parsing system

Goal:

-   Debug print of the AST itself should be able to be used to generate the AST.
    That is, the debug print should be a lossless representation of the original
    source text.

Example:

```rust
// original source text
let x = 1 + 2;

// debug print of the AST
let x = 1 + 2;

// AST
```

[ ] - Create a proc macro within the log crate that will log the function name,
file name, and line number of the function that is being called. This will be
useful for debugging and tracing the execution of

[ ] - choose between `tracing` and `log` for logging

[ ] - Extract the `arena` crate into an external `stampede` crate: generic
typed, thread-safe arena for low-level allcation features along with high-level
APIs for managing memory - [ ] `stampede` - [ ] **Typed** indices into the arena

-   [ ] Figure out how shared contexts will work

    -   References:
        -   https://tmandry.gitlab.io/blog/posts/2021-12-21-context-capabilities/
        -   https://jam1.re/blog/thoughts-on-contexts-and-capabilities-in-rust

-   [ ] Generic Interning, similar to `internment` crate, but with a more generic
        API and more features (i.e. `Arc` and `Rc` support, etc.). Same API
        exposed for both **multi-threaded** and **single-threaded** contexts.

-   [ ] Automate versioning / tagging and general release cycle to a certain
        extent (i.e. have cadence of releases)

-   [ ] Add support for compiling to wasm/wasi
-   [ ] grammar correctness via LALRPOP
-   [ ] transition from `lalrpop` to `ungrammar`

<!-- language scope syntax for interop with other languages, defines cross-language boundary -->
<!-- the variables within the _language scope_ are visible to the above module -->

-   [ ] Idea: language interop is very declarative and simple (i.e. `c { ... }`),
        but the actual implementation of the interop is very complex. This is
        because the interop is implemented in the compiler itself. Instead, the
        interop should be implemented in a separate crate, and the compiler should
        be able to use that crate to implement the interop. This will make the
        compiler much more modular and easier to maintain.

-   interesting idea maybe, not sure how to deal with types across boundaries
    though along with a number of other issues

# Example of language interop

```rust
// File: sort.leaf

// language interop
c {
    // C code
    // quicksort
    int quicksort(int *arr, int left, int right) {
        int i = left, j = right;
        int tmp;
        int pivot = arr[(left + right) / 2];

        /* partition */
        while (i <= j) {
            while (arr[i] < pivot)
                i++;
            while (arr[j] > pivot)
                j--;
            if (i <= j) {
                tmp = arr[i];
                arr[i] = arr[j];
                arr[j] = tmp;
                i++;
                j--;
            }
        };

        /* recursion */
        if (left < j)
            quicksort(arr, left, j);
        if (i < right)
            quicksort(arr, i, right);
    }
} // end of language interop

// language interop
rust {
    // Rust code
    fn sort(sort_type: &str, arr: &mut [i32]) {
        match sort_type {
            "quicksort" => quicksort(arr, 0, arr.len() - 1), // calls C function
            "mergesort" => mergesort(arr, 0, arr.len() - 1), // calls Rust function
            "bubblesort" => bubblesort(arr), // calls Python function
            _ => panic!("Invalid sort type"),
        }
    }
}

python {
    // import from above scope
    import sort.SortKind as sort_kind;

    // Python code
    def bubblesort(arr):
        n = len(arr)

        # Traverse through all array elements
        for i in range(n):
            # Last i elements are already in place
            for j in range(0, n - i - 1):
                # traverse the array from 0 to n-i-1
                # Swap if the element found is greater
                # than the next element
                if arr[j] > arr[j + 1]:
                    arr[j], arr[j + 1] = arr[j + 1], arr[j]
}

// #[derive(Debug, Clone, Copy)]
// trait as a first class feature in language

// Example of a trait

// #[derive(Debug, Clone, Copy)] // derive is a macro, not sure about this
trait Sort {
    fn sort(&mut self);
}

// Example of a struct

#[derive(Debug, Clone, Copy)]
pub enum SortKind {
    QuickSort,
    MergeSort,
    BubbleSort,
}

// use the language interop...
fn main() {
    // insertion sort is fairly fast on sorted inputs
    arr /* (inferred) : [i32] */ := [1, 2, 3, 4, 5, 6, 10, 9, 8, 7];

    // quicksort (implemented in C)
    println!("quicksort implemented in C")
    sort("quicksort", &mut arr.clone())
    println!("{:?}", arr)

    // mergesort (implemented in Rust)
    println!("mergesort implemented in Rust")
    sort("mergesort", &mut arr.clone())

    // bubblesort (implemented in Python)
    println!("bubblesort implemented in Python")
    sort("bubblesort", &mut arr.clone())
}
```

# Goal: Really good dependency analysis and management

Inspired by Go, no need to have a `Cargo.toml` file, just have a `go.mod` like
file. The `go.mod` file looks like this:

`leaf.pkg`

```mod
pkg github.com/leaf-lang/leaf

leaf 0.34.3 // version of the leaf compiler

// dependencies
// these are the dependencies of the current package
// the dependencies are specified by their name and version
// the version can be a specific version, or a range of versions
// the version can be a git commit hash, or a git tag
// the version can be a local path to a directory

dep github.com/leaf-lang/leaf-stdlib 0.23.4 // direct
```

Create build script to auto increment state of how many builds have occurred.

Auto-update minor every x number of builds. start at 100-1000 range and tune
accordingly. Want avg cadence of two-three weeks.

TODO: add snapshotting for dep-graph (i.e. auto update) via `cargo-make`
