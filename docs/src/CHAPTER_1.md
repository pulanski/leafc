# Chapter 1: _Lexical Analysis_

During the phase of [**lexical analysis**](https://www.wikiwand.com/en/Lexical_analysis)
(_located in the **[leafc_lexer](https://docs.rs/leafc_lexer)** crate_) the
source code is broken into a **stream of tokens**.

Each token is a combination of its **token kind** (e.g. `IDENTIFIER`), **span**
(e.g. `0..19`), and the **actual raw bytes** (e.g. `🦀ferris_the_crab`).

## Example

Source code:

```

```

Tokens:

```



The **token kind** is used later in the **syntax** for the **parser's grammar**.
For example, if the token type is **identifier**, later in the compilation
lifecycle, that token needs to be looked up in the **symbol table** to determine
its value. Depending on the context, the output of the lexer is either a
**lossless** or **lossy representation** of the source code. There are two
**distinct use cases** in mind for the lexer:

- **IDEs** and **editors** that need to provide **syntax highlighting** and
**auto-completion**. In this case, the lexer needs to be **lossless** and provide
**detailed information** about the source code.
- **Compilers** that need to provide **error messages** and **diagnostics**. In
this case, the lexer needs to be **lossy** and provide **minimal information**
about the source code such as to **optimize performance** and **streamline the
compilation process**.

Images below show the **lossless** and **lossy** representations of some example
source code. [image](../mandelbrot.png)
```
