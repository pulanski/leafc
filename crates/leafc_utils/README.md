# leafc_utils

This crate contains common utilities used throughout the `leafc` compiler. It is
**not intended** to be used by any other crates.

Utilities include:

<!-- - `Diagnostic` and `DiagnosticBag` for reporting errors and warnings -->

<!-- - `StringTable` for storing strings in a single location -->

**Location** utilities:

-   `Span` for representing a **span** (e.g. `start..end`) in the source code
-   `LineColumn` for representing a **location in a file** (e.g. `line:column`)
-   `Location` for representing a **location** in the source code of a **program** (e.g. `file:line:column`). This is the most common type of location used in the compiler.

and more...
