# Extern, `extern`

The `extern` keyword is used to declare an external function or block of code, most
commonly used to **call C functions**.

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
