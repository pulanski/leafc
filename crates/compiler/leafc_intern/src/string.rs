#[cfg(feature = "multi-threaded")]
use std::sync::Arc;

use lasso::Rodeo;

#[cfg(feature = "multi-threaded")]
#[derive(Debug, Clone)]
#[repr(transparent)]
pub struct StrInterner(Arc<ThreadedRodeo<Spur, Hasher>>);

// TODO: add support for non-threaded interning
// #[cfg(not(feature = "multi-threaded"))]
// #[derive(Debug, Clone)]
// #[repr(transparent)]
// pub struct StrInterner(Rodeo<Spur, Hasher>);
