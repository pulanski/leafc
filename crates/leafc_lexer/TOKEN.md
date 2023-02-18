# A **formal definition** of all the tokens seen by the **Leafc compiler**

**Tokens** are _primitive productions_ in the **grammar** of the source
language. The tokens themselves are defined via a set of **regular
expressions**. The **lexer** is responsible for taking the source code
and converting it into a token stream for the **parser** to consume.

You can find the full list of tokens and more relevant documentation
at the [Leafc documentation](https://leaf-lang.github.io/leafc/leafc_lexer/enum.Token.html).
