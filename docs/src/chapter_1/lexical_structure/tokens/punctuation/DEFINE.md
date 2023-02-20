## DEFINE, `:=`

A **define symbol** `:=` is used to define a variable. Variables are used to store values in Leaf. They can be used to store any type of value, including numbers, strings, booleans, and more. Variables cannot be uninitialized, as they must be defined before they can be used.

**Usage**:

-   **Variable Definition**.

### Regex

The regex for `:=` is:

```regex
:=
```

### Examples

```leaf
a, b := 1, 2

mut c := a + b

assert_eq(c, 3)

c -= 1

assert_eq(c, 2)
```
