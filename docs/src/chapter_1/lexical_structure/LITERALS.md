# Literals

Literals are the source code representation of values, they include **integer
literals**, **float literals**, **string literals**, **boolean literals**, and
more. They are used to represent values of different types. For example, the
literal `1` represents the value `1` of the type `i32`. The literal `1.0`
represents the value `1.0` of the type `f32`. The literal `"hello"` represents
the value `"hello"` of the type `String`.

**NOTE**: The type of a literal is determined by the context in which it is
used. For example, the literal `1` is of type `i32` when used in an expression
that expects an `i32`, but it is of type `f32` when used in an expression that
expects an `f32`.

<br>

<!-- TODO: update descriptions -->

| Literal                   | Description                                                                 |
| ------------------------- | --------------------------------------------------------------------------- |
| [Rune](#RUNE)             | A **rune**.                                                                 |
| [String](#STRING)         | A **string** literal.                                                       |
| [Raw String](#RAW_STRING) | A **raw string** (i.e. a string literal that **does not proccess** escapes) |
| [Integer](#INTEGER)       | An **integer** literal.                                                     |
| [Float](#FLOAT)           | A **float** literal.                                                        |
| [Lifetime](#LIFETIME)     | A **lifetime**.                                                             |

<a name="RUNE"></a>

{{ #include tokens/literals/RUNE.md }}

<br>

<a name="STRING"></a>

{{ #include tokens/literals/STRING.md }}

<br>

<a name="RAW_STRING"></a>

{{ #include tokens/literals/RAW_STRING.md }}

<br>

<a name="INTEGER"></a>

{{ #include tokens/literals/INTEGER.md }}

<br>

<a name="FLOAT"></a>

{{ #include tokens/literals/FLOAT.md }}

<br>

<a name="LIFETIME"></a>

{{ #include tokens/literals/LIFETIME.md }}

<br>

**TODO**: Add superscript literals (e.g. `1²`)
