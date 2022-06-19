//! # Art
//!
//! A library for modeling artistic concepts.

// To remove the internal organization from the public API, we can modify
// the art crate code to add pub use statements to re-export the items at
// the top level.
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
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
    pub fn mix(_c1: PrimaryColor, _c2: PrimaryColor) {
    }
}
