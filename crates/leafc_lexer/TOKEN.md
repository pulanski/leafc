# A **formal definition** of all the tokens seen by the **Leafc compiler**

**Tokens** are *primitive productions* in the **grammar** of the source
language. The tokens themselves are defined via the use of **regular expressions**
using [Logos](https://docs.rs/logos/latest/logos/), a lexical-analyzer
generator similar to [Lex](https://en.wikipedia.org/w/index.php?title=Lex_programming_tool&redirect=no).
If you are unfamiliar with **regular expressions**, you can read about
them [here](https://en.wikipedia.org/wiki/Regular_expression).
Additionally, if you are unfamiliar with some of the commonly
used **lexical-analyzer generators** used within the construction
of compilers, you can read about them [here](https://en.wikipedia.org/wiki/Comparison_of_parser_generators).

Source input can be broken down into the following kinds of tokens:

<!-- * [Keywords] -->
* **Keywords**
* **Identifiers**
* **Literals**
* **Lifetimes**
* **Punctuation**
* **Delimiters**
<!-- * [Identifiers][identifier]
* [Literals](#literals)
* [Lifetimes](#lifetimes-and-loop-labels)
* [Punctuation](#punctuation)
* [Delimiters](#delimiters) -->

You can find the full list of tokens and more relevant documentation
at the [Leafc documentation](https://leaf-lang.github.io/leafc/leafc_lexer/enum.Token.html).
