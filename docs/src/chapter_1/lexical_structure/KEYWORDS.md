# Keywords

Within Leaf, there are a number of keywords that are **reserved** for use by the language. These keywords **cannot** be used as **identifiers**, and will cause a **compile-time error** if they are used as such.

## Reserved Keywords

The [following keywords](./tokens/keywords/reserved/RESERVED.md) are **reserved for future use** by the language:

| Reserved Keyword | Description                                                          |
| ---------------- | -------------------------------------------------------------------- |
| `abstract`       | Used to **define** an **abstract class** or **method**.              |
| `async`          | Used to **define** an **asynchronous function**.                     |
| `await`          | Used to **wait for the completion** of an **asynchronous function**. |
| `extern`         | Used to **define** an **external function**.                         |
| `final`          | Used to **define** a **final variable**.                             |
| `is`             | Used to **check** if a value is of a **certain type**.               |

## Keywords

The [following keywords](#KEYWORDS) are **reserved for use** by the language:

| Keyword    | Description                                                                               |
| ---------- | ----------------------------------------------------------------------------------------- |
| `and`      | Used to **combine** two **boolean expressions**.                                          |
| `as`       | Used to **cast** a value to a **different type**.                                         |
| `break`    | Used to **break out of a loop**.                                                          |
| `const`    | Used to **define** a **constant**.                                                        |
| `continue` | Used to **continue to the next iteration** of a **loop**.                                 |
| `defer`    | Used to **defer the execution** of a **block of code** until the **current scope exits**. |

<br>

<a name="KEYWORDS"></a>

{{ #include tokens/keywords/AS.md }}

<!--             | `deinit`                                                     | Used to define a deinitializer. | -->

| Keyword     | Description                                                                   |
| ----------- | ----------------------------------------------------------------------------- |
| `as`        | Used to cast a value to a different type.                                     |
| `break`     | Used to break out of a loop.                                                  |
| `case`      | Used to match a value against a pattern.                                      |
| `catch`     | Used to catch an error thrown by a `try` block.                               |
| `class`     | Used to define a class.                                                       |
| `continue`  | Used to continue to the next iteration of a loop.                             |
| `default`   | Used to specify a default case in a `switch` statement.                       |
| `defer`     | Used to defer the execution of a block of code until the current scope exits. |
| `do`        | Used to define a loop.                                                        |
| `else`      | Used to specify an alternative block of code.                                 |
| `enum`      | Used to define an enumeration.                                                |
| `extension` | Used to define an extension.                                                  |
| `false`     | Used to represent a boolean value of `false`.                                 |
| `for`       | Used to define a loop.                                                        |
| `func`      | Used to define a function.                                                    |
| `if`        | Used to define a conditional block of code.                                   |
| `import`    | Used to import a module.                                                      |
| `in`        | Used to specify a range.                                                      |
| `init`      | Used to define an initializer.                                                |
| `is`        | Used to check if a value is of a certain type.                                |
| `let`       | Used to define a constant.                                                    |
| `nil`       | Used to represent a value of `nil`.                                           |
| `operator`  | Used to define an operator.                                                   |
| `protocol`  | Used to define a protocol.                                                    |
| `return`    | Used to return a value from a function.                                       |
| `self`      | Used to refer to the current instance.                                        |
| `static`    | Used to define a static property or method.                                   |
| `struct`    | Used to define a structure.                                                   |
| `switch`    | Used to define a switch statement.                                            |
| `throw`     | Used to throw an error.                                                       |
| `true`      | Used to represent a boolean value of `true`.                                  |
| `try`       | Used to define a try block.                                                   |
| `typealias` | Used to define a type alias.                                                  |
| `var`       | Used to define a variable.                                                    |
| `while`     | Used to define a loop.                                                        |
| `where`     | Used to specify a constraint on a generic type.                               |
| `with`      | Used to define a scope.                                                       |
| `yield`     | Used to yield a value from a generator.                                       |
