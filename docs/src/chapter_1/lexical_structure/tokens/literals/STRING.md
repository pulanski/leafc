## String

A **string** is a sequence of zero or more bytes enclosed in double quotes. The
value of a string literal is the sequence of bytes represented by the string.
The escape sequences observed within string literals follow the same syntactic
patterns as are found in [**Go**](https://go.dev/ref/spec#String_literals). A
string may contain **any valid UTF-8 sequence**, except for the NUL byte. A
string may **span multiple lines** and can contain
**any number of consecutive backslashes**, **double quotes**, **newlines**, and
**carriage returns**.

### Regex

```regex
r#"b?"(\\.|[^\\"])*""#
```

### Examples

```
"hello"

"hello\nworld"

"bonjour\nle monde"

"adiós\tel mundo"
```
