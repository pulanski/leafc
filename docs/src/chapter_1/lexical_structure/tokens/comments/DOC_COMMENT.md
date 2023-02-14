## Doc Comment

A **doc comment** is a sequence of characters that is not part of the source code.
They are intended to be used for **documentation** purposes. Depending on the
**context**, doc comments are be **ignored** or **included** in the **output**.
For example, if the **context** is a **compiler**, doc comments are **ignored**,
however, if the **context** is a **documentation generator** or **IDE**, doc
comments are **included** in the **output**.

### Regex

```regex
///[^\r\n]*
///[^\r\n]*
```

### Examples

```
/// This is a doc comment
```
