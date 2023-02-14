## Raw String

A **raw string** is a string literal that may span multiple lines and may
contain **any character**, including newlines and double quotes, **without escaping**.

### Regex

```regex
r#"r#[^#]*#"#
```

### Examples

```
r#"hello"#

r#"hello\nworld"#

r#"tu dit "bonjour", je dit "bonjour"
en français?"#
```
