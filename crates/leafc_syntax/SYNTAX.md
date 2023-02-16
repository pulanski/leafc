# Syntax in leafc

## Introduction

Inspired by the syntaxes of [Rust](https://doc.rust-lang.org/nightly/grammar.html), [Julia](https://docs.julialang.org/en/v1/base/base/), and [Go](https://go.dev/ref/spec), leafc's syntax is designed with an emphasis on **readability** and **simplicity**.

## Architecture

The overall architecture of **leafc's frontend** (e.g. the `leafc_parser`, and `leafc_syntax` crates) is _heavily inspired_ by many of the techniques used in [rust-analyzer](https://github.com/rust-lang/rust-analyzer/blob/master/docs/dev/syntax.md), which was in turn inspired by the work done in Swift's [libsyntax](https://tree-sitter.github.io/tree-sitter/) and C#'s [Roslyn](https://learn.microsoft.com/en-us/dotnet/csharp/roslyn-sdk/). If you're interested in learning more of the motivation behind the design, I highly recommend watching the series, [Explaining rust-analyzer](https://www.youtube.com/playlist?list=PLhb66M_x9UmrqXhQuIpWC5VgTdrGxMx3y) on YouTube.

## Syntax

The `leafc_syntax` crate inside `leafc` is a light-weight wrapper around the `rowan` crate which can be found [here](https://docs.rs/rowan/0.14.2/rowan/). The `rowan` crate is a **tree-sitter**-like library for parsing and traversing syntax trees. It is designed to be **light-weight**, **fast**, and **memory-efficient**. It is also **extensible** and **easy to use**.

In general, syntax trees possess the following properties:

-   **Immutable**: Syntax trees are **immutable**, meaning that once a syntax tree is created, it **cannot be modified**. This is a good thing, because it means that we can **share** them **between threads** without worrying about synchronization.
-   **Persistent**: Syntax trees are **persistent**, meaning that when a syntax tree is modified, the original syntax tree is **not modified**. Instead, a **new syntax tree** is created which shares as much data as possible with the original syntax tree. This allows us to **cache syntax trees** and reuse them whenever possible.
-   **Lossless**: Syntax trees are **lossless**, meaning that the original source code can be **recovered** from a syntax tree. This allows us to use syntax trees for things like refactoring and code generation, lending them useful for scenarios such as **code completion** in an **IDE context** and **code formatting** in a **linter context**.

This is a 50-thousand foot view of the syntax tree architecture in leafc. If you're interested in learning more about the design, I highly recommend reading the [rust-analyzer docs](https://github.com/rust-lang/rust-analyzer/blob/master/docs/dev/syntax.md).
