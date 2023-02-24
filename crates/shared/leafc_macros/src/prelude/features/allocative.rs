/// [`allocative`] feature declaration, which is used to enable the
/// [`Allocative`] trait.
#[macro_export]
macro_rules! ALLOCATIVE_FEATURE_DECL {
    () => {
        #[cfg(feature = "allocative")]
        use allocative::Allocative;
    };
}
