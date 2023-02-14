## Comment

A **comment** is a sequence of characters that is not part of the source code.
They can be used to provide additional information about the source code as they
are useful for documentation purposes. Depending on the **context**, comments
are **ignored** or **included** in the **output**. For example, if the
**context** is a **compiler**, comments are **ignored**. If the **context** is a
**documentation generator** or **IDE**, comments are **included** in the **output**.

### Regex

```regex
//[^\r\n]*
//[^\r\n]*
```

### Examples

```
// This is a comment
```
