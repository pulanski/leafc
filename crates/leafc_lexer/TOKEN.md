# A **formal definition** of all the tokens seen by the **Leafc compiler**

**Tokens** are _primitive productions_ in the **grammar** of the source
language. The tokens themselves are defined via the use of **regular expressions**
using [Logos](https://docs.rs/logos/latest/logos/), a lexical-analyzer
generator similar to [Lex](https://en.wikipedia.org/w/index.php?title=Lex_programming_tool&redirect=no).
If you are unfamiliar with **regular expressions**, you can read about
them [here](https://en.wikipedia.org/wiki/Regular_expression).
Additionally, if you are unfamiliar with some of the commonly
used **lexical-analyzer generators** used within the construction
of compilers, you can read about them [here](https://en.wikipedia.org/wiki/Comparison_of_parser_generators).

You can find the full list of tokens and more relevant documentation
at the [Leafc documentation](https://leaf-lang.github.io/leafc/leafc_lexer/enum.Token.html).

## Tokens

| Token        | Description                       | Example                |
| ------------ | --------------------------------- | ---------------------- |
| `Comment`    | A comment in the source code.     | `// This is a comment` |
| `Whitespace` | Whitespace in the source code.    | ` `                    |
| `Newline`    | A newline in the source code.     | `\n`                   |
| `Identifier` | An identifier in the source code. | `foo`                  |
| `Integer`    | An integer in the source code.    | `42`                   |
| `Float`      | A float in the source code.       | `3.14`                 |
| `String`     | A string in the source code.      | `"Hello, world!"`      |
| `Char`       | A character in the source code.   | `'a'`                  |
