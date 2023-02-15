## Infinity, _∞_

The **mathematical constant** _infinity_, `∞`. **Infinity** is a mathematical concept that represents an unbounded number. It is used to represent the limit of a sequence of numbers, or the limit of a function at a point. It's used in many different use cases, including:

- Calculus
- Statistics
- and more...

### Regex

The regex for `infinity` is:

```regex
([+-]?∞(16|32)?|[+-]?(inf|Inf|INF)(16|32)?)
```

### Examples

```leaf
assert_eq(∞, 1/0)
assert_eq(ℯ^-Inf32, 0)
```
