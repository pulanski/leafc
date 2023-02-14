## Lifetime

Inspired by both [Cyclone and Rust](https://pling.jondgoodwin.com/post/cyclone/),
a **lifetime** is a name that is used to define the **scope** of a variable or reference.
A **lifetime** is a sequence of one or more ASCII letters and underscores,
starting with a `'` (e.g. `'a`, `'static`, `'foo`, `'🦀` etc.).

### Regex

```regex
r#"'[\p{XID_Start}\p{Emoji_Presentation}][\p{XID_Continue}\p{Emoji_Presentation}]*"#
```

## Examples

```
'a

'🦀

'foo

'static
```
