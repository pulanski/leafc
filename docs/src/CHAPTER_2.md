# Chapter 2: _Syntax Analysis_

During the phase of [**syntax analysis**](https://www.wikiwand.com/en/Parsing) (_located in the **[`leafc_parser`](https://docs.rs/leafc_parser)** crate_) the **stream of tokens** is broken into a [**syntax tree**][ast explorer]. For example, the source code:

```rust
fn main() {
    🦀ferris_the_crab := "a crab named ferris"
    println!("Hello, {}!", 🦀ferris_the_crab)
}
```

is broken into the following **syntax tree**:

```text
ROOT
└── FN_DEF
    ├── FN_KW @ [0..2] fn
    ├── IDENTIFIER @ [3..7] main
    ├── L_PAREN @ [7..8] (
    ├── R_PAREN @ [8..9] )
    └── BLOCK
        ├── L_BRACE @ [10..11] {
        ├── LET_STMT
        │   ├── IDENTIFIER @ [16..35] 🦀ferris_the_crab
        │   ├── DEFINE @ [36..38] :=
        │   └── STRING @ [39..60] "a crab named ferris"
        ├── EXPR_STMT
        │   ├── IDENTIFIER @ [65..72] println
        │   ├── BANG @ [72..73] !
        │   ├── L_PAREN @ [73..74] (
        │   ├── STRING @ [74..86] "Hello, {}!"
        │   ├── COMMA @ [86..87] ,
        │   └── IDENTIFIER @ [88..107] 🦀ferris_the_crab
        │   └── R_PAREN @ [107..108] )
        └── R_BRACE @ [109..110] }
```

### An quick aside on a bit of **language theory**...

Within the **typical software development lifecyle** (_compiled, that is_), there are two distinct phases:

-   **Compile time**: the source code is transformed into an executable binary

    -   At this stage, the compiler **analyzes** the code, **checks it for syntax errors**, and **generates machine-readable code** that can be _executed by the computer_. The compiler performs **static analysis**, which means it can **detect** and **report errors** _before_ the program is _actually run_. Compile-time errors prevent the program from being successfully compiled and result in error messages. This means that, if your code builds, it has a **_certain degree_ of correctness** which is guaranteed by the compiler. This is a **desirable feature** of any programming language, because it **reduces** the **number of bugs** that can be introduced into the program. It also **reduces** the **number of run-time errors** that can occur, because the compiler is **able** to **detect** and **report** a **number of errors** that the **interpreter**, _by definition_, is **unable** to detect, such as **syntax errors**, **type errors**, and **undefined variables**.Compile-time safety checks are important because they **prevent** the program from **crashing** at _runtime_, and as such are a **desireable feature** of any programming language.

-   **Run time**: the executable binary is executed

    -   Run time, on the other hand, refers to the period during which a program is _actually executed_ by a computer. At this stage, the executable code is **loaded into memory**, and the computer starts to **execute the instructions** in the code. The program's behavior during run time is **determined** by the **input it receives** and the **interactions it has** with the **system it is running on**. Run-time errors are errors that occur **while** the program is _being executed_, and can include things like **logic errors**, **input errors**, and **memory errors**, among others. Run-time errors are **usually undesirable** because they **cause** the program to **crash** or **produce incorrect results**. This can end up causing a number of issues, such **security issues** or a hit on the overall **user experience** with the software.

With theses two phases in mind, Leaf's semantics are designed to **maximize** the **compile-time safety** of the language, while attempting to **minimizing run-time errors**, wherever possible. It's an important to note, you can't **prevent** all run-time errors. No matter what language you use, whether it's [**Java**][java homepage], or something _more esoteric_ like [**Starlark**][starlark homepage], you can (_and probably will_) write some kind of a logical error. Leaf attempts to eliminate certain kinds of errors <!-- add link here to what kinds of errors are of importance / are design goals --> by addressing them with certain **design decisions**, such as:

<!-- TODO: add -->

Compile-time safety is an underappreciated feature because the **compiler** is **able** to **perform** a **number of checks** on the **source code** that the **interpreter** is **unable** to perform. For example, the **compiler** is **able** to **detect** and **report** a **number of errors** that the **interpreter** is **unable** to detect, such as:

-   **Syntax errors**: the compiler is able to **detect** and **report** **syntax errors** in the source code, such as **missing parentheses** or **missing semicolons**, while an interpreter is unable to catch this error **until run time**.
-   **Type errors**: the compiler is able to **detect** and **report type errors** in the source code, such as **attempting to add** a **string** to an **integer**. The interpreter is **unable to detect** these errors before the program is run for the end user, and, as such, **reports** these errors at **run time**.
-   **Undefined variables**: the compiler is able to **detect** and **report undefined variables** in the source code, while an interpreter is unable to catch this error **until run time**.

> **NOTE**: These problems are not to unique to Leaf and can exist in virtually any programming language, but in **dynamic languages** (such as **Python** or **JavaScript**) these errors are **usually** **detected** and **reported** at **run time**, or require additional tooling (i.e. other static analyzers) to provide certain **correctness guarantees**.

<!-- Footer -->

If you're interested in learning more about the **syntax analysis** phase, you can check out the **[`leaf_explorer`][leaf_explorer]** tool, which is a **web-based** tool that allows you to **interactively** explore the **syntax tree** of a given piece of Leaf source code.

[ast explorer]: https://astexplorer.net/
[leaf_explorer]: https://leaf.dev/explorer
[java homepage]: https://www.java.com/en/
[starlark homepage]: https://bazel.build/rules/language
