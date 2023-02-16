# Introduction

<!-- TODO written in markdown and compiled into a [PDF]( -->

This book is the primary reference for the **Leaf language**. It is a
**work in progress** and will be updated as the language evolves.
The book is written in an attempt to _formalize_ the **language's design** and
provide a reference for future contributors behind the design and internal architecture. As such, the book is divided into the
following sections:

-   Chapters that informally describe the language's **syntax** and **grammar**.

-   Chapters that informally describe the language's **semantics**.

-   Chapters that informally describe the language's **standard library**.

-   Chapters that informally describe the **compiler and runtime architecture**, including
    the compiler's **intermediate representation (IR)**, **linkage model**, and **debugging utilities**.

> **NOTE**: Chapters that are **bolded** are considered **stable** and are ready for use, while chapters that are _**italicized**_ are considered _**unstable**_ and are _**subject to change**_

View the [rendered version of the book](https://leaf-lang.org/book), [download the PDF](https://leaf-lang.org/book.pdf), or [dive into the source code](https://github.com/pulanski/leafc).

## What this Book is Not

This book is **not** a tutorial on **how to use the language** or an **introduction to the language** itself. If you are looking for a tutorial on how to use the language, please refer to the [Leaf Programming Language](https://leaf-lang.org/reference) instead. This book is intended rather for those who are interested in the **design** and **architecture** behind the language's **compiler and runtime**.

## Built on the Shoulders of Giants

Many of the ideas in Leaf, the language itself, are in large part inspired by the following languages:

-   [Rust](https://www.rust-lang.org/)
    -   **Trait-based** type system
-   [Julia](https://julialang.org/)
    -   **Expressive** syntax for **mathematical** and **scientific** computing
-   [Go](https://golang.org/)
    -   Emphasis on **simplicity** and **ease of use** in the surface language design
    <!-- - [Elm](https://elm-lang.org/) -->

### Architecture

In addition to the above languages, Leaf is also inspired by the following projects when it comes to the **compiler and runtime architecture**:

-   [Nushell](https://www.nushell.sh/)
    -   Design of the language's **REPL** environment
-   [rust-analyzer](https://rust-analyzer.github.io/)
    -   Design of the language's **syntax** and **parsing** subsystems as well as **IDE** integration and **debugging** utilities
        -   **LSP** server
        -   **Syntax tree** visualization
        -   **Semantic highlighting**
    -   Approach to **syntax tree construction** and **parsing** _(using [rowan](https://docs.rs/rowan))_, inspired by both **Swift**'s [libsyntax]() and **C#**'s [Roslyn]()
        -   **Full-fidelity** syntax trees
        -   **Structural sharing** of syntax nodes _(multiple nodes can share the **same underlying data**)_
-   [rustc](https://github.com/rust-lang/rust)

    -   The **incremental compilation** subsystem _(using [salsa](https://github.com/salsa-rs/salsa))_ which allows for **incremental** compilation of the language's **source code** into **machine code**. This allows for **faster** compilation times as the compiler can **reuse** the results of previous compilations. The following subsytems are included in the **incremental compilation** piepline:

        -   **Lexing** a **character stream** into a **token stream**
            -   **Never relexes** a file _(or any string)_ that **has not changed**
        -   **Parsing** a **token stream** into a **syntax tree**
            -   **Never reparses** a file _(or any token stream)_ that **has not changed**
        -   **Name resolution** _(using [chalk](https://github.com/rust-lang/chalk))_
            -   **Never re-resolves** a syntax tree that has not changed
            <!-- TODO: continue here -->

        Since all of these operations are **transitive dependencies** of one another, ultimately derived from the **source code** of the program, the compiler can **reuse** the results of previous compilations for **faster** compilation times, as no work is required to construct a **typed syntax tree** which has been **type checked** and has **resolved** all **names**, **paths**, and **types**. This is a **powerful concept** and **huge improvement** over the **traditional** approach found with many batch compilers, which require some degree of **recompilation**. We may not need to **recompile** the **entire** program from **scratch** every time a **single** file is **modified**, but rather **only the parts** of the program which have **changed**. As such, if you, for example, add a **documentation comment** to help improve the **readability** of your code, the compiler **won't** need to recompile the program, as the **AST** it sees **has not changed**. However, if you **modify** a **function** or **type** definition, then the compiler **will** need to recompile the program, but only the parts which have changed.

### Goals

-   **Ownership** and **borrowing** semantics for memory management via **regions** and **lifetimes**, allowing for a **high level abstraction** of _low level_ memory management
    _(originally inspired by ideas from the [Cyclone](https://cyclone.thelanguage.org/) project)_
    <!-- TODO: explore more about how things work in Zig, might find some ideas interesting -->

> This book expects that the reader has a **high-level understanding** of **compiler design**, but not necessarily a **deep** understanding of typical components found in the **architecture** of **today's compilers**. If you're familiar with the traditional phases of a compiler, such as **lexing**, **parsing**, **type checking**, **code generation**, and **linking**, then you should be able to follow along with the book. If you're not familiar with these concepts, then you may want to read up on them before diving in, but it's **not strictly necessary**. You're more than free to go through the book and **learn as you go**. If you're interested in learning more about the **traditional phases** of a compiler, then I recommend the following resources:
>
> Books commonly found at university computer science departments:
>
> -   [**Compilers: Principles, Techniques, and Tools**](https://www.amazon.com/Compilers-Principles-Techniques-Tools-2nd/dp/0321486811) by **Aho, Lam, Sethi, and Ullman** _(the **bible** of compilers, aka the **Dragon Book**)_.
> -   [**Engineering a Compiler**](https://www.amazon.com/Engineering-Compiler-Keith-Cooper/dp/012088478X) by **Keith Cooper** and **Linda Torczon**. This book is a **great** resource for learning about **practical techniques** used in **compiler design**.
