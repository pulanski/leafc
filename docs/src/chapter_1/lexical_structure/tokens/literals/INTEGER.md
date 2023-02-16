## Integer

An **integer** is a sequence of one or more decimal digits representing a
non-negative integer value. An **optional prefix** can be used to set the base
of the integer:

| Prefix | Base        |
| ------ | ----------- |
| `0x`   | Hexadecimal |
| `0o`   | Octal       |
| `0b`   | Binary      |

Additionally, an **optional suffix** may be used to set the size of the integer at
**compile time**:

| Suffix  | Size                                                   |
| ------- | ------------------------------------------------------ |
| `u8`    | 8-bit                                                  |
| `i8`    | 8-bit                                                  |
| `u16`   | 16-bit                                                 |
| `i16`   | 16-bit                                                 |
| `u32`   | 32-bit                                                 |
| `i32`   | 32-bit                                                 |
| `u64`   | 64-bit                                                 |
| `i64`   | 64-bit                                                 |
| `u128`  | 128-bit                                                |
| `i128`  | 128-bit                                                |
| `usize` | 32-bit or 64-bit, depending on the target architecture |
| `isize` | 32-bit or 64-bit, depending on the target architecture |

### Regex

```regex
0[xX][0-9a-fA-F_]+(u8|i8|u16|i16|u32|i32|u64|i64|u128|i128|usize|isize)?
|0[oO][0-7_]+(u8|i8|u16|i16|u32|i32|u64|i64|u128|i128|usize|isize)?
|0[bB][01_]+(u8|i8|u16|i16|u32|i32|u64|i64|u128|i128|usize|isize)?
|[0-9][0-9_]*(u8|i8|u16|i16|u32|i32|u64|i64|u128|i128|usize|isize)?
```

### Examples

The following are all **valid integer literals**:

```ignore
123

123i32

123u32

123_u32

0xff

0xff_u8

0X01_f32

0B________1

0usize

0xBadFace

0XdEaDBeEf

0x_67_7a_2f_cc_40_c6

170141183460469231731687303715884105727
```
