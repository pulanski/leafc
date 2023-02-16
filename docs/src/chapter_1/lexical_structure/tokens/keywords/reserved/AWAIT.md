# Await, `await`

The `await` keyword is used to **call** an **async function** and **suspend** the current **execution context** until it returns.

## Regex

```regex
await // English
| esperar // Spanish
| attendre // French
| erwarten // German
| aguardam // Portuguese
| attendere // Italian
| vente // Danish
| avvente // Norwegian
| vänta // Swedish
| wachten // Dutch
| odottaa // Finnish
| Ждите // Russian
| 待つ // Japanese
| 等待 // Chinese
| 기다리다 // Korean
| kusubiri // Swahili
```

## Examples

```leaf
async fn foo() {
    println!("Hello, world!")
}

fn main() {
    foo()
}
```
