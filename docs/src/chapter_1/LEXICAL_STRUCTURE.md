# Lexical Structure

In regards to **lexical structure**, the Leaf language has an interesting feature:
**polyglot syntax**. This means that the source language itself can be written
in and translated between **multiple spoken languages**.

> **NOTE**: The **default language** is determined based on the **locale** of the system on which the Leaf compiler is being run. The **locale** may also be determined by the `LANG` environment variable. For example, if the `LANG` environment variable is set to `en_US.UTF-8`, the **default language** is **English**. On the other hand, if the `LANG` environment variable is set to `fr_FR.UTF-8`, the **default language** is **French**. This setting can be overridden via a number of different methods including _command line flag_, _environment variable_, _configuration file_.

The **default language** is used to determine both the **legal syntax** of the source code as well as the language used within the compiler's **error messages**. For example, if the default language is **English**, the following code is valid:

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

On the other hand, if the current language was **French**, the **inverse** would be **true**.

## Token Groups

Tokens within the **Leaf language** are grouped into the **following collections** seen
below. Each token group is defined by a **series of regular expressions**.

<br>

| [General Tokens](./lexical_structure/GENERAL_TOKENS.md)                |
| ---------------------------------------------------------------------- |
| [`Whitespace`](./lexical_structure/GENERAL_TOKENS.md#whitespace)       |
| [`Identifier`](./lexical_structure/GENERAL_TOKENS.md#identifier)       |
| [`Lexical Error`](./lexical_structure/GENERAL_TOKENS.md#lexical-error) |

<br>

| [Comments](./lexical_structure/COMMENTS.md)                            |
| ---------------------------------------------------------------------- |
| [`Comment`](./lexical_structure/COMMENTS.md#comment-comment)           |
| [`Documentation Comment`](./lexical_structure/COMMENTS.md#doc-comment) |

<!-- TODO: refactor to this -->
<!-- | Line Comment                                |
| Block Comment                               |
| Documentation Comment                       | -->

<br>

| [Literals](./lexical_structure/LITERALS.md)                |
| ---------------------------------------------------------- |
| [`Rune`](./lexical_structure/LITERALS.md#RUNE)             |
| [`String`](./lexical_structure/LITERALS.md#STRING)         |
| [`Raw String`](./lexical_structure/LITERALS.md#RAW_STRING) |
| [`Integer`](./lexical_structure/LITERALS.md#INTEGER)       |
| [`Float`](./lexical_structure/LITERALS.md#FLOAT)           |
| [`Lifetime`](./lexical_structure/LITERALS.md#LIFETIME)     |

<br>

<!-- TODO: add more mathematical symbols of interest -->
<!-- | Epsilon                                                             | -->

| [Mathematical Symbols](./lexical_structure/MATHEMATICAL_SYMBOLS.md)                      |
| ---------------------------------------------------------------------------------------- |
| [Pi, `π`](./lexical_structure/MATHEMATICAL_SYMBOLS.md#PI)                                |
| [Euler's Number, `𝑒`](./lexical_structure/MATHEMATICAL_SYMBOLS.md#EULER)                 |
| [Phi, `φ`](./lexical_structure/MATHEMATICAL_SYMBOLS.md#PHI)                              |
| [Tau, `τ`](./lexical_structure/MATHEMATICAL_SYMBOLS.md#TAU)                              |
| [Catalan's Constant, `𝑘`](./lexical_structure/MATHEMATICAL_SYMBOLS.md#CATALAN)           |
| [Euler-Mascheroni Constant, `𝛾`](./lexical_structure/MATHEMATICAL_SYMBOLS.md#EULERGAMMA) |
| [Infinity, `∞`](./lexical_structure/MATHEMATICAL_SYMBOLS.md#INFINITY)                    |
| [Not-a-Number, `NaN`](./lexical_structure/MATHEMATICAL_SYMBOLS.md#NAN)                   |

<br>

| [Reserved Keywords](./lexical_structure/KEYWORDS.md)                            |
| ------------------------------------------------------------------------------- |
| [`abstract`](./lexical_structure/tokens/keywords/reserved/RESERVED.md#ABSTRACT) |
| [`async`](./lexical_structure/tokens/keywords/reserved/RESERVED.md#ASYNC)       |
| [`await`](./lexical_structure/tokens/keywords/reserved/RESERVED.md#AWAIT)       |
| [`case`](./lexical_structure/tokens/keywords/reserved/RESERVED.md#CASE)         |
| [`extern`](./lexical_structure/tokens/keywords/reserved/RESERVED.md#EXTERN)     |
| [`final`](./lexical_structure/tokens/keywords/reserved/RESERVED.md#FINAL)       |
| [`is`](./lexical_structure/tokens/keywords/reserved/RESERVED.md#IS)             |
| [`let`](./lexical_structure/tokens/keywords/reserved/RESERVED.md#LET)           |
| [`priv`](./lexical_structure/tokens/keywords/reserved/RESERVED.md#PRIV)         |
| [`proc`](./lexical_structure/tokens/keywords/reserved/RESERVED.md#PROC)         |
| [`switch`](./lexical_structure/tokens/keywords/reserved/RESERVED.md#SWITCH)     |

<br>

| [Keywords](./lexical_structure/KEYWORDS.md)                  |
| ------------------------------------------------------------ |
| [`and`](./lexical_structure/KEYWORDS.md#AND)                 |
| [`as`](./lexical_structure/KEYWORDS.md#AS)                   |
| [`break`](./lexical_structure/KEYWORDS.md#BREAK)             |
| [`const`](./lexical_structure/KEYWORDS.md#CONST)             |
| [`continue`](./lexical_structure/KEYWORDS.md#CONTINUE)       |
| [`defer`](./lexical_structure/KEYWORDS.md#DEFER)             |
| [`do`](./lexical_structure/KEYWORDS.md#DO)                   |
| [`dyn`](./lexical_structure/KEYWORDS.md#DYN)                 |
| [`else`](./lexical_structure/KEYWORDS.md#ELSE)               |
| [`enum`](./lexical_structure/KEYWORDS.md#ENUM)               |
| [`fallthrough`](./lexical_structure/KEYWORDS.md#FALLTHROUGH) |
| [`false`](./lexical_structure/KEYWORDS.md#FALSE)             |
| [`fn`](./lexical_structure/KEYWORDS.md#FN)                   |
| [`for`](./lexical_structure/KEYWORDS.md#FOR)                 |
| [`if`](./lexical_structure/KEYWORDS.md#IF)                   |
| [`impl`](./lexical_structure/KEYWORDS.md#IMPL)               |
| [`in`](./lexical_structure/KEYWORDS.md#IN)                   |
| [`loop`](./lexical_structure/KEYWORDS.md#LOOP)               |
| [`match`](./lexical_structure/KEYWORDS.md#MATCH)             |
| [`missing`](./lexical_structure/KEYWORDS.md#MISSING)         |
| [`mod`](./lexical_structure/KEYWORDS.md#MOD)                 |
| [`move`](./lexical_structure/KEYWORDS.md#MOVE)               |
| [`mut`](./lexical_structure/KEYWORDS.md#MUT)                 |
| [`not`](./lexical_structure/KEYWORDS.md#NOT)                 |
| [`or`](./lexical_structure/KEYWORDS.md#OR)                   |
| [`pkg`](./lexical_structure/KEYWORDS.md#PACKAGE)             |
| [`pub`](./lexical_structure/KEYWORDS.md#PUB)                 |
| [`return`](./lexical_structure/KEYWORDS.md#RETURN)           |
| [`self`](./lexical_structure/KEYWORDS.md#SELF_VALUE)         |
| [`Self`](./lexical_structure/KEYWORDS.md#SELF_TYPE)          |
| [`static`](./lexical_structure/KEYWORDS.md#STATIC)           |
| [`struct`](./lexical_structure/KEYWORDS.md#STRUCT)           |
| [`super`](./lexical_structure/KEYWORDS.md#SUPER)             |
| [`trait`](./lexical_structure/KEYWORDS.md#TRAIT)             |
| [`true`](./lexical_structure/KEYWORDS.md#TRUE)               |
| [`type`](./lexical_structure/KEYWORDS.md#TYPE)               |
| [`unsafe`](./lexical_structure/KEYWORDS.md#UNSAFE)           |
| [`use`](./lexical_structure/KEYWORDS.md#USE)                 |
| [`where`](./lexical_structure/KEYWORDS.md#WHERE)             |
| [`while`](./lexical_structure/KEYWORDS.md#WHILE)             |
| [`yield`](./lexical_structure/KEYWORDS.md#YIELD)             |

<!-- | [`union`](./lexical_structure/KEYWORDS.md#UNION)             | -->

<br>

| [Punctuation](./lexical_structure/PUNCTUATION.md)    |
| ---------------------------------------------------- |
| [`(`](./lexical_structure/PUNCTUATION.md#LPAREN)     |
| [`)`](./lexical_structure/PUNCTUATION.md#RPAREN)     |
| `[`](./lexical_structure/PUNCTUATION.md#LBRACK)      |
| `]`](./lexical_structure/PUNCTUATION.md#RBRACK)      |
| `{`](./lexical_structure/PUNCTUATION.md#LBRACE)      |
| `}`](./lexical_structure/PUNCTUATION.md#RBRACE)      |
| `,`](./lexical_structure/PUNCTUATION.md#COMMA)       |
| `;`](./lexical_structure/PUNCTUATION.md#SEMICOLON)   |
| `:`](./lexical_structure/PUNCTUATION.md#COLON)       |
| `.`](./lexical_structure/PUNCTUATION.md#DOT)         |
| `..`](./lexical_structure/PUNCTUATION.md#DOTDOT)     |
| `...`](./lexical_structure/PUNCTUATION.md#DOTDOTDOT) |
| `@`](./lexical_structure/PUNCTUATION.md#AT)          |
| `#`](./lexical_structure/PUNCTUATION.md#HASH)        |
| `$`](./lexical_structure/PUNCTUATION.md#DOLLAR)      |
| `?`](./lexical_structure/PUNCTUATION.md#QUESTION)    |
| `!`](./lexical_structure/PUNCTUATION.md#EXCLAMATION) |
| `=`](./lexical_structure/PUNCTUATION.md#EQUAL)       |
| `<`](./lexical_structure/PUNCTUATION.md#LT)          |
| `>`](./lexical_structure/PUNCTUATION.md#GT)          |
| `-`](./lexical_structure/PUNCTUATION.md#MINUS)       |
| `+`](./lexical_structure/PUNCTUATION.md#PLUS)        |
| `*`](./lexical_structure/PUNCTUATION.md#STAR)        |
| `/`](./lexical_structure/PUNCTUATION.md#SLASH)       |
| `%`](./lexical_structure/PUNCTUATION.md#PERCENT)     |
| `&`](./lexical_structure/PUNCTUATION.md#AMPERSAND)   |
