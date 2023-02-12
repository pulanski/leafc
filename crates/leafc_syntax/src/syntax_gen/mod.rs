//! Handles the underpinnings for **declaratively defining** the **grammar**
//! for the various **syntaxes** of the **language** and then **procedurally generating** the
//! associated **APIs** for the those syntaxes which are used by various **dependent subsystems**
//! throughout the compiler.
//!
//! **NOTE**: This module is heavily inspired by the success of [**syntax generation**](https://rust-analyzer.github.io/blog/2020/10/24/introducing-ungrammar.html) within **rust-analyzer** and the credit for the **original
//! ideas** behind this work should go to the **rust-analyzer** team. If you are interested, I would recommend
//! reading the linked post as it does a great job of explaining the **motivations** behind
//! **syntax generation** and the **benefits** it provides.
//!
//! # Future potential setup described below...
//!
//! ## For now, we just convert **concrete syntax trees** to **abstract syntax trees** using the `From` trait,
//! e.g. `AstNode::from()
//!
//! # Syntax Generation
//!
//! In general, the **syntax generation** process is **split** into **two** **distinct** **phases**:
//!
//! 1. **Declarative Grammar Definition**: The **grammar** of the **language** is **defined** in a
//!   **declarative** manner. This is done via the [`ungrammar`]() crate which is a **DSL** for
//!   **defining** the **grammar** of a **language**. This **grammar** is then **compiled** to a
//!
//! 2. **Procedural Code Generation**: The **grammar** is then **used** to **generate** the **code**
//!  which is used by various **dependent subsystems** of the compiler (e.g. the **parser**, **IDE**, etc.)
//!
//! ## Comparison to rust-analyzer
//!
//! This approach diverges from that taken by **rust-analyzer**, as the projects have **different goals**.
//! **rust-analyzer** is a **language server** which is **used** by **IDEs** to provide **rich IDE
//! features**. As such, it is **desirable** for the **syntax** of the **language** to be **defined**
//! using a **concrete syntax tree** (CST) which is **used** by the **parser**, as opposed to an
//! **abstract syntax tree** (AST) which is **used** by the **compiler**. This is because the **IDE**
//! and the **compiler** have **different requirements** for the **syntax tree**. The **IDE** requires
//! a **concrete syntax tree** which is **used** to **provide** the **rich IDE features** such as
//! **code completion**, **hovering**, etc. The **compiler** on the other hand requires an **abstract
//! syntax tree** which is **used** to **progressively lower** the **source code** into a **simpler,
//! lower-level representation**. This is because the **compiler** is **interested** in the **semantics**
//! of the **source code** rather than its **syntax**. As such, `syntax_gen` is **designed** to
//! **generate** both the **concrete syntax tree** and the **abstract syntax tree** from the **grammar**,
//! empowering the ability to have a **single source of truth** for the **syntax tree** system of the
//! **language**. Additionally, `syntax_gen` goes a step further by **guaranteeing** that the **grammar**
//! is **correct** by **compiling** it to a **parser generator** which is **used** to **check** for
//! **ambiguities** in the **grammar** and other parsing-related issues.
//!
//! ## Requirements / Goal
//!
//! Have a **single source of truth** for the **syntax tree** system of the **language**.
//! This means that the **grammar** specified in `leaf.ungram` should be the **only**
//! place where the language's **syntax** is defined. This **single source of truth**
//! should be **used** to **generate** the **data structures** which are used to
//! represent the different **syntax trees** of the **language**. The approach should be
//! modular and not hand off the **responsibility** of **defining** the **syntax** of the
//! **language** to a **parser generator**, or the **responsibility** of **parsing** the
//! **source code** to a **parser generator**. Instead, the **grammar** should be
//! **compiled** to a **parser generator** which is **used** to **check** for **ambiguities**
//! in the **grammar** and other parsing-related issues. This is done to **guarantee**
//! that the **grammar** is **correct**.
//!
//! ------------------
//!
//! **TODO**: in the future, this single source of truth should be updated to a syntax
//! generator with correctness guarantees for the associated grammar
//! (e.g. no ambiguity, etc.). This is done to shore up the core weakness of the
//! current syntax generator. It is not guaranteed to be correct. This is comes from lack of
//! **correctness guarantees** which come with using [**ungrammar**](https://github.com/rust-analyzer/ungrammar) and the goal would be to create a layer which compiled to ungrammar
//! that would provide these desired correctness guarantees.
//!
//! To do this though, we would need to generate effectively a _"no-op"_ parser generator
//! via the LALRPOP parser generator which would be used to verify the correctness of
//! the grammar as it evolves. This would allow for a "correct by construction" approach
//! while also having the infrastructure for really nice APIs for the syntax tree
//! system.
//!
//! ------------------
//!
//!  This includes both the
//! **concrete syntax trees** and the **abstract syntax trees**. From the **grammar** (and
//! associated syntax tree system) is where the implementation details for various subsystems
//! within the compiler diverge. For example, the **parser** uses the **grammar** to **generate**
//! either the **concrete syntax trees** or the **abstract syntax trees**. Then, based on the context, either
//! the **concrete syntax trees** or the **abstract syntax trees** are **used** for the
//! associated dependent subsytem. For example, the **IDE** and **Linting** subsystems
//! would be interested on working with the source **directly** via the **concrete syntax trees**.
//! On the other hand, the **compiler** subsystem would be interested in working with the
//! **abstract syntax trees**, as a primary goal within a typical compiler is to **progressively
//! lower** the **source code** into a **simpler, lower-level representation**, and to do this,
//! we shave off any **unnecessary** information from the **source code**.
//!
//!
//! ## Overview
//!
//! The **syntax tree** system is **divided into two parts**:
//!
//! 1. **Concrete syntax trees** (CSTs) are **generated** from the **grammar** specified in `leaf.ungram`.
//!
//! 2. **Abstract syntax trees** (ASTs) are **generated** from the **CSTs**.
//!
//! **Concrete syntax trees** are **full-fidelity** representations of the **source code**, whereas
//! **abstract syntax trees** are **simplified** representations of the **source code** which typically
//! **remove** any **unnecessary** information from the **source code** (e.g. whitespace, comments, etc.).
//!
//! **TODO** - include diagram for the differences below
//!
//! e.g. the **source code**: `let x = 1;`
//!
//! **CST**:
//!
//! ```text
//! SourceFile {
//!    statements: [
//!       LetStatement {
//!         name: Identifier {
//!          name: "x",
//!        },
//!       value: Literal {
//!         value: 1,
//!      },
//!   },
//! ],
//! }
//! ```
//!
//! **AST**:
//!
//! ```text
//! SourceFile {
//!   statements: [
//!    LetStatement {
//!     name: Identifier {
//!      name: "x",
//!   },
//!  value: Literal {
//!  value: 1,
//! },
//! },
//! ],
//! }
//! ```
//!
//! # Motivation for the **syntax tree** system
//!
//! One of the seminal components to any development ecosystem is the **tooling** around it.
//! If it takes too long to **write** code, or if the tooling is **lacking**, then
//! it becomes an unpleasant experience to **develop** software using that approach, whatever
//! it may be. When designing the language, an important goal was to try and have
//! **good tooling** from the **start**. This means rather than recreating the **wheel**
//! for every **tool** that is needed, it is important to **reuse** existing **components**
//! and **libraries** which are **well designed** and **well tested**
//!
//! The **syntax tree** system **represents** the **core** of the **language**, and over time
//! it will **evolve** to become **more and more complex**, whether that means adding new features,
//! or removing ones with which issues are found. In an effort to **simplify** the
//! **development process** of the **language**, but still have a **robust** and **flexible**
//! **API** for the syntax tree system, it is important to have a system which
//! **automates** the process of the writing the **data structures** which are used to
//! represent the **syntax trees** of the **language**. This **automated generation process**
//! should be **flexible** enough to **adapt** to **changes** in the **syntax tree** system (e.g.
//! updating how variable declaration works as the language evolves).
//!
//! ### Example
//!
//! Say the design for variable declaration was to be changed _for some reason_ from a **Go**
//! inspired syntax of
//!
//! `x := 5`,
//!
//! to a **Rust**-inspired syntax of
//!
//! `let x = 5`
//!
//! or instead to a **C**-inspired syntax of
//!
//! `int x = 5`
//!
//! A change like this might require a significant amount of **manual work** to update **both**
//! the **concrete syntax trees** and **abstract syntax trees** which ared used by the various
//! dependent subsystems of the compiler and associated tooling. The goal here was do
//! minimize the amount of **manual work** required to **update** the **syntax tree** system
//! and minimize **"effective blast radius"** those changes would have on the rest of the
//! compiler as it **evolves** over time.
//!
//! and not require a significant amount of **manual work** to **update** the
//! **data structures** which are used by the rest of the compiler. With this in mind, the
//! ultimate goal is a single source of truth for both the **AST** utilized by the **compiler**
//! itself and the **CST** utilized by the **language server** via the **grammar**.
//!
//! ## Benefits of the **syntax tree** system
//!
//! The **syntax tree** system is **flexible** and **robust**, and is **designed** to **adapt**
//! to **changes** in the language as it **evolves** over time (as with any new language,
//! changes will most likely be made to its **syntax*). The **syntax tree** makes introducing
//! those new changes less of a hassle, as it is abstracted away into a **relatively automated**
//! process for defining the underpinning data structures within the rest of the compiler.
//!
//! ## Approach
//!

#![allow(unused)] // TODO: remove This

pub mod input;

pub mod utils;

pub mod sourcegen;
