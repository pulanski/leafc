# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2023-02-21

### Bug Fixes

-   Fix failing doctests
-   Remove local sccache reference
-   Update rust crate quote to 1.0.23

### Documentation

-   Refactor docs for tokens from source code into book and then reference docs in source
-   Add more docs for various mathematical symbols
-   Flesh out docs more for lexical structure
-   Refactor to macros for token docs and add more docs

### Features

-   Initial scaffolding
-   Start to scaffold out build infrastructure
-   Begin work on repl logo
-   Begin to add tokens to the lexer
-   Add more polyglot keyword tokens
-   Finish polyglot keyword tokens
-   Finish implementing literals along w testing
-   Finish adding tokens for representing mathematical consts
-   Begin work on the syntax_gen subsystem
-   Begin adding scaffolding for languages and associated cfg
-   Add more generation for syntax kinds
-   Continue work on moving docs to book
-   Add fallthrough keyword along with more scaffolding for syntax
-   Begin scaffolding out errors, warnings, general diagnostics
-   Add more components to the ungrammar syntax
-   Add more implementation details to span and location
-   Begin adding string interning capabilities to the system

### Miscellaneous Tasks

-   Update rust crate clap to 4.1.6
-   Update rust crate llvm-sys to 160.0.2
-   Update rust crate once_cell to 1.17.1
-   Update rust crate rug to 1.19.1
-   Update rust crate serde_json to 1.0.93
-   Update rust crate smol_str to 0.1.24

### Refactor

-   Minimize the verbosity of cli doc comments
-   Refactor builder structs from derive-builder to typedbuilder

### Styling

-   Init trunk and fix linting issues

### Ci

-   Add initial ci for gh-actions
-   Temporarily remove support for windows in ci, windows suck

<!-- The Leaf Authors -->
