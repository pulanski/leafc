## Rune

A **rune** is a single character enclosed in single quotes. The value of a rune
literal is the Unicode code point value of the character enclosed in the quotes
(e.g. `'本'` is a rune). The data type for a rune literal is `rune`.

### Regex

```regex
b?'[^']*'
```

### Examples

```ignore
'本'

b'abc'

'👍'
```
