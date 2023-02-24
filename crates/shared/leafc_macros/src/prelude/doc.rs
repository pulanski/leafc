/// Macro defining the documentation for the `README.md` file in the crate root.
#[macro_export]
macro_rules! CRATE_README {
    () => {
        include_str!("../README.md")
    };
}
