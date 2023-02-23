# Isn't, `isn't`

The `isn't` keyword is used to **check** if a **value** is **not** of a **specific type**.

## Regex

```regex
isn't
| no es
| n'est pas
| is niet
| är inte
| er ikke
| ei ole
| не
| ではない
| 不是
| 아니야
| si siyo
```

## Examples

```leaf

// example for checking if a variable isn't a certain type

a := 1

// syntactic sugar for a match expression on a type

// rust equivalent
// looks like this:
// match TypeOf(a) { // TODO: check for what exact rust looks like
//     i32 => print("a is an i32"),
//     _ => print("a isn't an i32")
// }

if a isn't i32 {
    print("a isn't an i32")
} else {
    print("a is an i32")
}

// change the type of a to a String

a = "Hello, World!"
```
