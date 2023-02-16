## Identifier

An **identifier** is a sequence of one or more **Unicode letters** or **digits**
or **underscores** or **emoji**. The first character of an identifier must be
a **Unicode letter** or **underscore** or **emoji** (e.g. `a`, `_`, `本`, `🦀`).

### Regex

```regex
_?[\p{XID_Start}\p{Emoji_Presentation}][\p{XID_Continue}\p{Emoji_Presentation}]*
```

### Examples

```ignore
a

_abc

本

🦀
```
