//! # Cargo Doc
//!
//! `cargo_doc` is a collection of utilities to make performing certain
//! calculations more convenient.
//! //! # Art
//!
//! A library for modeling artistic concepts.

/// Adds two to the number given.
///
/// # Examples
///
/// ```
/// use crago_doc::add_two;
///
/// let arg = 5;
/// let answer = add_two(arg);
///
/// assert_eq!(7, answer);
/// ```
pub fn add_two(x: i32) -> i32 {
    x + 2
}

pub mod kinds {
    /// The primary colors according to the RYB color model.
    #[derive(PartialEq)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    #[derive(PartialEq)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        match c1 == PrimaryColor::Blue && c2 == PrimaryColor::Red {
            true => SecondaryColor::Green,
            false => SecondaryColor::Orange,
        }
    }
}
