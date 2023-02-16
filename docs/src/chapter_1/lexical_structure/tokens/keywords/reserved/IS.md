# Is, `is`

The `is` keyword is used to **check** if a **value** is of a **specific type**.

## Regex

```regex
extern // English, Danish, Norwegian, Swedish, German, Dutch
| externo // Spanish, Portuguese
| externe // French
| esterno // Italian
| ulkoinen // Finnish
| внешний // Russian
| 外部 // Japanese
| 外部的 // Chinese
| 외부 // Korean
| nje // Swahili
```

## Examples

```leaf
extern fn foo() {
    char *greeting = "Hello, world!";
    printf("%s", greeting);
}

fn main() {
    foo()
}
```
