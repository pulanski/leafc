pub mod prelude;

// TODO: better separate prelude out

// Conditionally compile a list of items based on the given meta items.
//
// This is useful for conditional compilation of data structures and algorithms
// based on the features enabled in the crate. Facade over the internal
// workings of the data structures for different feature contexts.
//
// I.e. if the crate has a `no-std` feature, then the following
// encapsulation details would be hidden from the user:
//
// ```rust
// compile! {
//    if #[feature = "no-std"] {
//       pub use alloc::sync::Arc;
//    } else {
//       pub use std::sync::Arc;
//    }
// }
// ```
// #[macro_export]
// macro_rules! compile {
//     ($(
//         if #[$meta:meta] {
//             $($item:item)*
//         } $(else if #[$else_if_meta:meta] {
//             $($else_if_item:item)*
//         })* $(else {
//             $($else_item:item)*
//         })?
//     )+) => {
//         $(
//             $(
//                 #[cfg($meta)]
//                 $item
//             )*

//             compile!{
//                 @inner
//                 ( $meta, )
//                 $(else if #[$else_if_meta] {
//                     $( $else_if_item )*
//                 })* $(else {
//                     $( $else_item )*
//                 })?
//             }
//         )+
//     };

//     (@recurse
//         ($($prev_metas:tt)*)
//         ($new_meta:meta)
//         $($rem:tt)*
//     ) => {
//         compile!{
//             @inner
//             ($( $prev_metas )* $new_meta,)
//             $( $rem )*
//         }
//     };

//     (@inner
//         $prev_metas:tt
//         else if #[$meta:meta] {
//             $($else_if_item:item)*
//         }
//         $($rem:tt)*

//     ) => {
//         $(
//             #[cfg(all(not(any $prev_metas), $meta))]
//             $else_if_item
//         )*

//         compile! {
//             @recurse $prev_metas ($meta) $( $rem )*
//         }
//     };

//     (@inner
//         $prev_metas:tt
//         else {
//             $($else_item:item)*
//         }
//     )=>{
//         $(
//             #[cfg(not(any $prev_metas))]
//             $else_item
//         )*
//     };

//     (@inner ($($prev_metas:tt)*))=>{};
// }
