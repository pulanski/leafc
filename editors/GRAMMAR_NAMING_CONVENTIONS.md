# Grammar Naming Conventions (_IDE_)

Following naming conventions introduced in TextMate Grammars, the following naming conventions are used in the VSCode extension:

| Scope Name           | Description                    |
| -------------------- | ------------------------------ |
| `source.<language>`  | The root scope for a language. |
| `meta.<name>`        | A **meta scope**.              |
| `entity.<name>`      | An **entity**.                 |
| `variable.<name>`    | A **variable**.                |
| `constant.<name>`    | A **constant**.                |
| `keyword.<name>`     | A **keyword**.                 |
| `storage.<name>`     | A **storage type**.            |
| `string.<name>`      | A **string**.                  |
| `comment.<name>`     | A **comment**.                 |
| `invalid.<name>`     | An **invalid construct**.      |
| `support.<name>`     | A **support construct**.       |
| `punctuation.<name>` | A **punctuation character**.   |
| `markup.<name>`      | A **markup construct**.        |

## Meta Scopes

Meta scopes are used to group other scopes together. They are used to distinguish between different types of constructs.

> For example, in Objective-C there is a meta scope for the interface declaration of a class and the implementation, allowing the same tab-triggers to expand differently, depending on context.

## Comments

-   `comment` — for comments.
    -   `line` — line comments, we specialize further so that the type of comment start character(s) can be extracted from the scope.
    -   `double-slash` — // comment
    -   `number-sign` — # comment
    -   `character` — other types of line comments.
    -   `block` — multi-line comments like /_ … _/ and <!-- … -->.
    -   `documentation` — embedded documentation.

## Constants

These _effectively map_ to [**literals**][literals] as defined in the [**formal language reference**][leaf reference].

-   `constant` — for **constants**.
    -   `character` — for **character constants** (i.e. _single-quoted strings_ - `'a'`).
    -   `numeric` — for **numeric constants** (i.e. _integer_, _float_, _hex_, _octal_, _binary_ - `1`, `1.0`, `0x1`, `0o1`, `0b1`).
    -   `language` — for **language constants** (i.e. `true`, `false`, `Missing`).
    -   `other` — for **other constants**.

[literals]: https://en.wikipedia.org/wiki/Literal_(computer_programming)
[leaf reference]: https://leaf.dev/reference
