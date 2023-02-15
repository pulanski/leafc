## Not-a-Number, `NaN`

The **mathematical value** _not-a-number_, `NaN`. **NaN** is a mathematical concept that represents a value that is **not a number**. It is used to represent the result of an operation that does not have a valid number as a result.

### Regex

The regex for `NaN` is:

```regex
[+-]?(nan|NaN|NAN)(16|32)?
```

### Examples

```leaf
assert_eq(0/0, NaN)
assert_eq(Inf - Inf, NaN)
```
