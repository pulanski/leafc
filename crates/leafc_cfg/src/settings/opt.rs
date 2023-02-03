/// The **optimization level** to use when compiling the input file.
/// defaults to `OptLevel::None`
///
/// In general, there is an unavoidable tradeoff between compiler optimization and compile times. In
/// order to produce the most well-optimized executable, we pay the cost of higher compilation times.
/// However, this is not always desirable. For example, during development, we may want to compile our
/// code as quickly as possible, and we are not concerned with the performance of the executable. In
/// this case, we can use the [`OptLevel::None`] optimization level.
///
/// # Examples
///
/// ```rust
/// use leafc_cfg::settings::OptLevel;
///
/// // The default optimization level is `OptLevel::None` (i.e. no optimizations).
/// // The higher the optimization level, the more optimizations are performed.
/// assert!(OptLevel::None < OptLevel::O1);
/// assert!(OptLevel::O1 < OptLevel::O2);
/// assert!(OptLevel::O2 < OptLevel::O3);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum OptLevel {
    /// **No optimizations** are performed. This is the default optimization level.
    /// Useful for **debugging purposes** during development.
    None,

    /// **Basic optimizations passes** are performed.
    O1,

    /// More advanced optimizations are performed in combination to those performed in [`OptLevel::O1`].
    O2,

    /// The most advanced optimizations are performed in combination to those performed in [`OptLevel::O2`].
    /// This is the **most optimized** level.
    /// Useful for **production**.
    O3,
}
