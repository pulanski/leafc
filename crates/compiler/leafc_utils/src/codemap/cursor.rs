use crate::Location;

/// A **cursor** is a **pointer** to a
/// **location** in the **source code**.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cursor {
    /// The **location** of the cursor, i.e. the
    /// **file** and the **span**.
    location: Location,
    // The **source code** that the cursor is pointing to.
    // source:   Source,
}
