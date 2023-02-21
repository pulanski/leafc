# Lexer

The lexer is responsible for taking a string of text (e.g. a _character stream_) and turning it into a **stream of tokens**.

## Example

```rust
fn main() {
    println!("Hello, world!");
}
```

is lexed into:

```mermaid
graph LR
    1[fn] --> 2[main]
    2 --> 3[()]
    3 --> 4[->]
    4 --> 5[println]
    5 --> 6[!]
    6 --> 7[(]
    7 --> 8[\\"Hello, world!\\"] --> 9[)]
    9 --> 10[;]
    10 --> 11[}]
```
