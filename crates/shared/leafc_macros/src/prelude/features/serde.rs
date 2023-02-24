/// [`serde`] feature declaration, which is used to enable the
/// [`Serialize`] and [`Deserialize`] traits.
#[macro_export]
macro_rules! SERDE_FEATURE_USE_DECL {
    // take a list of strings like `"json", "yaml"`, convert them to types, and then use that type
    // in the `use` statement.
    () => {
        // check if the "json" feature is enabled
        #[cfg(feature = "serde")]
        {
            // check if the "json" feature is enabled
            #[cfg(feature = "json")]
            {
                // use the `serde_json` crate
                use serde_json::*;
            }

            // check if the "yaml" feature is enabled
            #[cfg(feature = "yaml")]
            {
                // use the `serde_yaml` crate
                use serde_yaml::*;
            }

            // check if the "bytes" feature is enabled
            #[cfg(feature = "bytes")]
            {
                // use the `serde_bytes` crate
                use serde_bytes::*;
            }
        }

        SERDE_FEATURE_USE_DECL_BASE!();
    };
}

macro_rules! SERDE_FEATURE_USE_DECL_BASE {
    () => {
        #[cfg(feature = "serde")]
        use serde::{
            Deserialize,
            Serialize,
        };
    };
}

// proc macro for this like the following:
// #[leafc_macros::serde_feature_impl]
// struct Foo;

// will generate the following:
//
// #[cfg(feature = "serde")]
// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
// #[serde(deny_unknown_fields)]
// #[serde(try_from = "Serde")]
// #[serde(into = "Serde")]
// struct Foo;

// TODO: make this using a proc_macro
// similar to the following api:
// #[leafc_macros::serde_feature_impl]
// #[leafc_macros::allocative_feature_impl]
//
// and then something for dependency injection w/ no-std (Not sure what the best
// way to do this is)
// #[leafc_macros::no_std_feature_impl]
//
// Simple macro w/ desired behavior:
//
// #[macro_export]
// macro_rules! SERDE_FEATURE_IMPL {
//     // #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
//     () => {
//         #[cfg(feature = "serde")]
//         #[derive(Serialize, Deserialize)]
//         #[serde(rename_all = "snake_case")]
//         #[serde(deny_unknown_fields)]
//         #[serde(try_from = "Serde")]
//         #[serde(into = "Serde")]
//     };
// }

// macro_rules! SERDE_FEATURE_TRY_FROM {
//     () => {
//         #[cfg(feature = "serde")]
//         impl TryFrom<Serde> for Self {
//             type Error = String;

//             fn try_from(serde: Serde) -> Result<Self, Self::Error> {
//                 Ok(Self {
//                     $(
//                         $field: serde.$field,
//                     )*
//                 })
//             }
//         }
//     };
// }

#[macro_export]
macro_rules! SERDE_JSON_USE_DECL {
    () => {
        #[cfg(feature = "serde")]
        use serde_json::{
            Result,
            Value,
        };
    };
}

#[macro_export]
macro_rules! SERDE_YAML_USE_DECL {
    () => {
        #[cfg(feature = "serde")]
        use serde_yaml::{
            Result,
            Value,
        };
    };
}
