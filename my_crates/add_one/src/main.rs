extern crate art;

// In cases where there are many nested modules, re-exporting the types at the
// top level with pub use can make a significant difference in the experience
// of people who use the crate.
// Creating a useful public API structure is more of an art than a science,
// and you can iterate to find the API that works best for your users.
// Choosing pub use gives you flexibility in how you structure your crate
// internally and decouples that internal structure from what you present
// to your users. Look at some of the code of crates you’ve installed to
// see if their internal structure differs from their public API.
use art::PrimaryColor;
use art::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}