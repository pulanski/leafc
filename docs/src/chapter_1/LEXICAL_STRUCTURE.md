# Lexical Structure

In regards to **lexical structure**, the Leaf language has an interesting feature:
**polyglot syntax**. This means that the source language itself can be written
in and translated between **multiple spoken languages**. The **default language**
is determined based on the **locale** of the system on which the Leaf compiler,
`leafc`, is running. For example, if the system's locale is set to `en_US.UTF-8`,
the default language is **English**. On the other hand, if the system's locale is
set to `fr_FR.UTF-8`, the default language is **French**. This setting can be
overridden via a number of different methods including _command line flag_,
_environment variable_, _configuration file_. The **default language** is used
to determine both the **legal syntax** of the source code as well as the
language used within the compiler's **error messages**. For example, if the
default language is **English**, the following code is valid:

```rust
fn main() {
    mut 🦀 := "ferris the crab"
    println!("Hello, {}!", 🦀)

    🦀 = "ferris le crabe"
    println!("Bonjour, {}!", 🦀)
}
```

But the following code is **invalid**:

```rust
fn principal() {
    mutable 🦀 := "ferris le crabe";
    ligne_d'impression!("Bonjour, {}!", 🦀);

    🦀 = "ferris le crabe";
    ligne_d'impression!("Bonjour, {}!", 🦀);
}
```

If the default language was **French**, the **inverse** would be **true**.

Within the lexer's purview is Leaf's **lexical structure**. This is the
**set of rules** that define the **valid** and **invalid tokens** that can
be found in the source code. The **lexical structure** is defined as a series
of **regular expressions** as detailed below. Feel free to skip this section if
you are not interested in the details of the lexer.

## Tokens

Each token created from the lexer is a combination of its **token kind** (e.g. `IDENTIFIER`),
**span** (e.g. `0..19`), and the **raw bytes** (e.g. `🦀ferris_the_crab`).

| Token Kind   | Span    | Raw Bytes          |
| ------------ | ------- | ------------------ |
| `IDENTIFIER` | `0..19` | `🦀ferris_the_crab` |

The **token kind** is used later in the **syntax** for the **parser's grammar**.
For example, if the token type is **identifier**, later in the compilation
lifecycle, that token needs to be looked up in the **symbol table** to
determine its value. Depending on the context, the output of the lexer is
either a **lossless** or **lossy representation** of the source code. There are
two **distinct use cases** in mind for the lexer:

- Batch compilation of source code

- IDE integration

## Token Groups

Tokens within the **Leaf language** are grouped into the **following collections** seen
below. Each token group is defined by a **series of regular expressions**.

<br>

| [General Tokens](./lexical_structure/GENERAL_TOKENS.md) |
| ------------------------------------------------------- |
| Whitespace                                              |
| Identifier                                              |
| Lexical Error                                           |

<br>

| [Comments](./lexical_structure/COMMENTS.md) |
| ------------------------------------------- |
| Comment                                     |
| Documentation Comment                       |
<!-- TODO: refactor to this -->
<!-- | Line Comment                                |
| Block Comment                               |
| Documentation Comment                       | -->

<br>

| [Literals](./lexical_structure/LITERALS.md) |
| ------------------------------------------- |
| Rune                                        |
| String                                      |
| Raw String                                  |
| Integer                                     |
| Float                                       |
| Lifetime                                    |

<br>

| [Mathematical Symbols](./lexical_structure/MATHEMATICAL_SYMBOLS.md) |
| ------------------------------------------------------------------- |
| Pi, `π`                                                             |
| Euler's Number, `𝑒`                                                 |
| Phi, `φ`                                                            |
| Tau, `τ`                                                            |
| Catalan's Constant, `𝑘`                                             |
| Euler-Mascheroni Constant, `𝛾`                                      |
| Infinity, `∞`                                                       |
| Not-a-Number, `NaN`                                                 |

<br>

| [Reserved Keywords](./lexical_structure/KEYWORDS.md) |
| ---------------------------------------------------- |
| `abstract`                                           |
| `as`                                                 |
| `async`                                              |
| `await`                                              |
| `extern`                                             |
| `final`                                              |
| `is`                                                 |


<!-- | Epsilon                                                             | -->
<!-- <br>

- [Keywords](./lexical_structure/KEYWORDS.md)

<br>

- [Punctuation](./lexical_structure/PUNCTUATION.md) -->
