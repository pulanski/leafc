use cfg_if::cfg_if;

cfg_if! {
    // features for specifying the string matching algorithm (i.e. Levenshtein, Damerau-Levenshtein, Hamming, Sift3)

    if #[cfg(feature = "levenshtein")] { // default string matching algorithm used
        pub use strsim::{levenshtein, normalized_levenshtein};
    }
    // else if #[cfg(feature = "damerau-levenshtein")] {
    //     pub use crate::string::distance::damerau_levenshtein::damerau_levenshtein;
    // } else if #[cfg(feature = "hamming")] {
    //     pub use crate::string::distance::hamming::hamming;
    // } else if #[cfg(feature = "sift3")] {
    //     pub use crate::string::distance::sift3::sift3;
    // } else {
    //     pub use crate::string::distance::levenshtein::levenshtein;
    // }
}
