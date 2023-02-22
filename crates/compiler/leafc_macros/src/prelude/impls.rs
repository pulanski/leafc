// TODO: Move to leafc_macros crate

#[macro_export]
macro_rules! impl_from {
    ($($from:ty => $to:ty),*) => {
        $(
            impl From<$from> for $to {
                fn from(from: $from) -> Self {
                    from.0
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! impl_from_ref {
    ($($from:ty => $to:ty),*) => {
        $(
            impl From<&$from> for $to {
                fn from(from: &$from) -> Self {
                    from.0
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! impl_from_mut {
    ($($from:ty => $to:ty),*) => {
        $(
            impl From<&mut $from> for $to {
                fn from(from: &mut $from) -> Self {
                    from.0
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! impl_from_ref_mut {
    ($($from:ty => $to:ty),*) => {
        $(
            impl From<(&mut $from,)> for $to {
                fn from(from: (&mut $from,)) -> Self {
                    from.0 .0
                }
            }
        )*
    };
}

#[macro_export]
macro_rules! impl_from_generic {
    ($($from:ty => $to:ty),*) => {
        $(
            impl<T> From<$from<T>> for $to<T> {
                fn from(from: $from<T>) -> Self {
                    from.0
                }
            }
        )*
    };
}
