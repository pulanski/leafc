## Float

A **floating point number** is a sequence of decimal digits representing a
floating point value. An **optional prefix** may be used to set the base of the
number

| Prefix | Base        |
| ------ | ----------- |
| `0o`   | Octal       |
| `0b`   | Binary      |
| `0x`   | Hexadecimal |

Additionally, an **optional suffix** can be used to set the size of the float at
compile time, with the default being `f64`:

| Suffix | Size   |
| ------ | ------ |
| `f32`  | 32-bit |
| `f64`  | 64-bit |

### Regex

```regex
// decimal
r#"[+-]?([0-9][0-9_]*)?\.([0-9][0-9_]*)?([eE][+-]?[0-9][0-9_]*)?(f32|f64)?"#

// hex
r#"[+-]?(0x|0X)[0-9a-fA-F][0-9a-fA-F_]*\.[0-9a-fA-F][0-9a-fA-F_]*([pP][+-]?[0-9][0-9_]?)?(f32|f64)?"#

// binary
r#"[+-]?0b[0-1][0-1_]*\.[0-1][0-1_]*([pP][+-]?[0-9][0-9_]?)?(f32|f64)?"#

// octal
r#"[+-]?0o[0-7][0-7_]*\.[0-7][0-7_]*([pP][+-]?[0-9][0-9_]?)?(f32|f64)?"#
```

### Examples

The following are all **valid float literals**:

```ignore
123.456

0.

.25

1.0E6

1.e+0

072.40

0.0f32

072.40

2.71828

0x0.0p0

123.0f32

123.0_f32

.12345E+5

0.15e+0_2

6.67428e-11
```
