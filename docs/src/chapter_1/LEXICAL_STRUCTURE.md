# Lexical Structure

In regards to **lexical structure**, the Leaf language has an interesting feature: **polyglot syntax**. This means that the source language itself can be written in and translated between **multiple spoken languages**. The **default language** is determined based on the system's **locale**. For example, if the system's locale is set to `en_US.UTF-8`, the default language is **English**. If the system's locale is set to `fr_FR.UTF-8`, the default language is **French**. This setting can be overridden via multiple methods (e.g. command line flag, environment variable, configuration file, etc.). The **default language** is used to **determine the syntax** of the source code and parse it accordingly. For example, if the default language is **English**, the following code is valid:

```rust
fn main() {
    mut 🦀 = "ferris the crab"
    println!("Hello, {}!", 🦀)

    🦀 = "ferris le crabe"
    println!("Bonjour, {}!", 🦀)
}
```

But the following code is **invalid**:

```rust
fn principal() {
    mutable 🦀 := "ferris le crabe"
    ligne_d'impression!("Bonjour, {}!", 🦀)

    🦀 = "ferris le crabe"
    ligne_d'impression!("Bonjour, {}!", 🦀)
}
```

Within the lexer's purview is Leaf's **lexical structure**. This is the **set of rules** that define the **valid** and **invalid** **tokens** that can be found in the source code. The **lexical structure** is defined as a series of **regular expressions** as detailed below. Feel free to skip this section if you are not interested in the details of the lexer.

# Tokens

The following is a list of all the **tokens** that can be found in the source code. Each token is a combination of its **token kind** (e.g. `IDENTIFIER`), **span** (e.g. `0..19`), and the **actual raw bytes** (e.g. `🦀ferris_the_crab`). The **token kind** is used later in the **syntax** for the **parser's grammar**. For example, if the token type is **identifier**, later in the compilation lifecycle, that token needs to be looked up in the **symbol table** to determine its value. Depending on the context, the output of the lexer is either a **lossless** or **lossy representation** of the source code. There are two **distinct use cases** in mind for the lexer:

{{ #include tokens/WHITESPACE.md }}
