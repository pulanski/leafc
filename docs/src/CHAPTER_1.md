# Chapter 1: _Lexical Analysis_

During the phase of [**lexical analysis**](https://www.wikiwand.com/en/Lexical_analysis)
(_located in the **[leafc_lexer](https://docs.rs/leafc_lexer)** crate_) the
source code is broken into a [**stream of tokens**](https://docs.rs/leafc_lexer/0.1.0/leafc_lexer/struct.TokenStream.html). For example, the source code:

```rust
fn main() {
    🦀ferris_the_crab := "a crab named ferris"
    println!("Hello, {}!", 🦀ferris_the_crab)
}
```

is broken into the following tokens:

```text
FN_KW @ [0..2] fn
IDENTIFIER @ [3..7] main
L_PAREN @ [7..8] (
R_PAREN @ [8..9] )
L_BRACE @ [10..11] {
IDENTIFIER @ [16..35] 🦀ferris_the_crab
DEFINE @ [36..38] :=
STRING @ [39..60] "a crab named ferris"
IDENTIFIER @ [65..72] println
BANG @ [72..73] !
L_PAREN @ [73..74] (
STRING @ [74..86] "Hello, {}!"
COMMA @ [86..87] ,
IDENTIFIER @ [88..107] 🦀ferris_the_crab
R_PAREN @ [107..108] )
R_BRACE @ [109..110] }
```

<br>

Within the lexer's purview is Leaf's **lexical structure**. This is the
**set of rules** that define the **valid** and **invalid tokens** that can
be found in the source code. These tokens are then used by the [**parser**](./CHAPTER_2.md) to
determine the **syntactic structure** of the source code (e.g. **function definitions**,
**expressions**, etc.). The lexer itself is defined as a series of **regular expressions**
as detailed below. Feel free to skip this section if you are not interested in the details
of the lexer and want to jump straight to the [**parser**](./CHAPTER_2.md).

## Tokens

Each token lexed by the lexer is a combination of its **token kind** (e.g. `IDENTIFIER`),
**span** (e.g. `0..19`), and the **raw bytes** (e.g. `🦀ferris_the_crab`).

| Token Kind   | Span    | Raw Bytes           |
| ------------ | ------- | ------------------- |
| `IDENTIFIER` | `0..19` | `🦀ferris_the_crab` |

The **token kind** is used later in the **syntax** for the **parser's grammar**.
For example, if the token type is **identifier**, later in the compilation
lifecycle, that token needs to be looked up in the **symbol table** to
determine its value. Depending on the context, the output of the lexer is
either a **lossless** or **lossy representation** of the source code. There are
two **distinct use cases** in mind for the lexer:

-   **Batch compilation** of source code
-   **IDE integration**
-   Source code **transformation**

As such, the example above is a **lossless representation** of the source code. In
the **batch compilation** use case, the **lossy representation** of the source
code is used, as the **compiler** is only interested in the parts of the source
code that are **relevant** to the compilation process (e.g. **function definitions**,
**variable declarations**, etc.) and not the **comments**, **whitespace**, and
**formatting**. However, in the **IDE integration** use case, the **lossless representation**
of the source code is used, as the **IDE** is interested in the **entirety** of the
source code, including **comments**, **whitespace**, and **formatting**. This allows the
**IDE** to provide **contextual information** to the user, such as **code completion**
and **documentation**, as well as **formatting** the source code to the user's preferences
and providing the ability to **refactor** _without breaking the code_ in a **large codebase**.
