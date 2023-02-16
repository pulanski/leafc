<!-- An example of potential documentation for syntax nodes and general syntactic structure -->

# As, `as`

The `as` keyword is used to cast a value to a different type.

```rust
let x: Int = 5
let y: Double = x as Double
```

## Regex

```regex
expression as type
```

## Parameters

| Parameter    | Description                         |
| ------------ | ----------------------------------- |
| `expression` | The expression to cast.             |
| `type`       | The type to cast the expression to. |
