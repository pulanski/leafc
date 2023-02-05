//! Terminal utilities.
//!
//! This module defines a collection of utilities for working with the terminal
//! including printing text to the terminal, clearing the terminal, and
//! formatting text.
//!
//! # Examples
//!
//! ```rust
//! use leafc_utils::terminal::{clear, horizontal_padding, vertical_padding};
//!
//! // Clear the terminal.
//! clear();
//!
//! // Print 10 blank lines.
//! vertical_padding(10);
//!
//! // Print 10 spaces.
//! horizontal_padding(10);
//! ```

/// Clears the terminal.
///
/// # Examples
///
/// ```rust
/// use leafc_utils::terminal::clear;
///
/// // Clear the terminal.
/// clear();
/// ```
// pub fn clear() {
//     use ansi_term::clear::All;

//     println!("{}", All);
// }

/// Prints a **specified number of blank lines** to the terminal.
/// This is useful for **formatting text vertically** when
/// creating a user interface.
///
/// # Examples
///
/// ```rust
/// use leafc_utils::terminal;
///
/// // Print 10 blank lines.
/// terminal::vertical_padding(10);
/// ```
pub fn vertical_padding(n: usize) {
    for _ in 0..n {
        println!();
    }
}

/// Prints a **specified number of spaces** to the terminal.
/// This is useful for **formatting text horizontally** when
/// creating a user interface.
///
/// # Examples
///
/// ```rust
/// use leafc_utils::terminal;
///
/// // Print 10 spaces.
/// terminal::horizontal_padding(10);
/// ```
pub fn horizontal_padding(n: usize) {
    for _ in 0..n {
        print!(" ");
    }
}

// TODO: Implement this.
// pub fn clear() {
//     use ansi_term::clear::All;

//     println!("{}", All);
// }
