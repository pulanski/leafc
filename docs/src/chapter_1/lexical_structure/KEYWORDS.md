# Keywords

Within Leaf, there are a number of keywords that are **reserved** for use by the language. These keywords **cannot** be used as **identifiers**, and will cause a **compile-time error** if they are used as such.

## Reserved Keywords

The [following keywords](./tokens/keywords/reserved/RESERVED.md) are **reserved** either for **future use** by the language or to disallow them in the surface syntax of the language. They are **currently not used** within the surface syntax of the language, but may be **used in the future**:

| Reserved Keyword                                              | Description                                                          |
| ------------------------------------------------------------- | -------------------------------------------------------------------- |
| [`abstract`](./tokens/keywords/reserved/RESERVED.md#ABSTRACT) | Used to **define** an **abstract class** or **method**.              |
| [`async`](./tokens/keywords/reserved/RESERVED.md#ASYNC)       | Used to **define** an **asynchronous function**.                     |
| [`await`](./tokens/keywords/reserved/RESERVED.md#AWAIT)       | Used to **wait for the completion** of an **asynchronous function**. |
| [`extern`](./tokens/keywords/reserved/RESERVED.md#EXTERN)     | Used to **define** an **external function**.                         |
| [`final`](./tokens/keywords/reserved/RESERVED.md#FINAL)       | Used to **define** a **final variable**.                             |
| [`is`](./tokens/keywords/reserved/RESERVED.md#IS)             | Used to **check** if a value is of a **certain type**.               |

## Keywords

<a name="KEYWORDS"></a>

The [following keywords](#KEYWORDS) are **reserved for use** by the language:

| Keyword     | Description                                                                               |
| ----------- | ----------------------------------------------------------------------------------------- |
| `and`       | Used to **combine** two **boolean expressions**.                                          |
| [`as`](#AS) | Used to **cast** a value to a **different type**.                                         |
| `break`     | Used to **break out of a loop**.                                                          |
| `const`     | Used to **define** a **constant**.                                                        |
| `continue`  | Used to **continue to the next iteration** of a **loop**.                                 |
| `defer`     | Used to **defer the execution** of a **block of code** until the **current scope exits**. |
| `do`        | Used to **define a loop**.                                                                |

<br>

<a name="AND"></a>

{{ #include tokens/keywords/AND.md }}

<br>

<a name="AS"></a>

{{ #include tokens/keywords/AS.md }}

<br>

<a name="BREAK"></a>

{{ #include tokens/keywords/BREAK.md }}

<br>

<a name="CONST"></a>

{{ #include tokens/keywords/CONST.md }}

<br>

<a name="CONTINUE"></a>

{{ #include tokens/keywords/CONTINUE.md }}

<br>

<a name="DEFER"></a>

{{ #include tokens/keywords/DEFER.md }}

<br>

<a name="DO"></a>

{{ #include tokens/keywords/DO.md }}

<br>

<a name="DYN"></a>

{{ #include tokens/keywords/DYN.md }}

<br>

<a name="ELSE"></a>

{{ #include tokens/keywords/ELSE.md }}

<br>

<a name="ENUM"></a>

{{ #include tokens/keywords/ENUM.md }}

<br>

<a name="FALLTHROUGH"></a>

{{ #include tokens/keywords/FALLTHROUGH.md }}

<br>

<a name="FALSE"></a>

{{ #include tokens/keywords/FALSE.md }}

<br>

<a name="FN"></a>

{{ #include tokens/keywords/FN.md }}

<br>

<a name="FOR"></a>

{{ #include tokens/keywords/FOR.md }}

<br>

<a name="IF"></a>

{{ #include tokens/keywords/IF.md }}

<br>

<a name="IMPL"></a>

{{ #include tokens/keywords/IMPL.md }}

<br>

<a name="IN"></a>

{{ #include tokens/keywords/IN.md }}

<br>

<a name="LOOP"></a>

{{ #include tokens/keywords/LOOP.md }}

<br>

<a name="MATCH"></a>

{{ #include tokens/keywords/MATCH.md }}

<br>

<a name="MISSING"></a>

{{ #include tokens/keywords/MISSING.md }}

<br>

<a name="MOD"></a>

{{ #include tokens/keywords/MOD.md }}

<br>

<a name="MOVE"></a>

{{ #include tokens/keywords/MOVE.md }}

<br>

<a name="MUT"></a>

{{ #include tokens/keywords/MUT.md }}

<br>

<a name="NOT"></a>

{{ #include tokens/keywords/NOT.md }}

<br>

<a name="OR"></a>

{{ #include tokens/keywords/OR.md }}

<br>

<a name="PACKAGE"></a>

{{ #include tokens/keywords/PACKAGE.md }}

<br>

<a name="PUB"></a>

{{ #include tokens/keywords/PUB.md }}

<br>

<a name="RETURN"></a>

{{ #include tokens/keywords/RETURN.md }}

<br>

<a name="SELF_TYPE"></a>

{{ #include tokens/keywords/SELF_TYPE.md }}

<br>

<a name="SELF_VALUE"></a>

{{ #include tokens/keywords/SELF_VALUE.md }}

<br>

<a name="STATIC"></a>

{{ #include tokens/keywords/STATIC.md }}

<br>

<a name="STRUCT"></a>

{{ #include tokens/keywords/STRUCT.md }}

<br>

<a name="SUPER"></a>

{{ #include tokens/keywords/SUPER.md }}

<br>

<a name="SWITCH"></a>

{{ #include tokens/keywords/SWITCH.md }}
