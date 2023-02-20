## AMPERSAND_EQ, `&=`

An `AMPERSAND_EQ` symbol is a `PUNCTUATION` symbol that represents the `&=` operator.

**Usage**:

-   **Bitwise AND Assignment**.

### Regex

The regex for `&=` is:

```regex
&=
```

### Examples

```leaf
// Variable definition often follows the pattern of
// declaring a set of immutable variables, followed by
// a set of mutable variables.
//
// This is done in an effort to improve readability and
// consistency across codebases.

c := 3              // immutable
mut a, b := 1, 2    // mutable

// mut a, b is just syntactic sugar for:
// mut { a, b } (i.e. a mutable tuple / block)

a &= b
b &= c

assert_eq((a, b), (0, 2))
```
