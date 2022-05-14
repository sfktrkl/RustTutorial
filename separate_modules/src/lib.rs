// So far, all the examples in this chapter defined multiple modules in one file.
// When modules get large, you might want to move their definitions to a separate
// file to make the code easier to navigate.
mod front_of_house;

// Note that the pub use crate::front_of_house::hosting statement in src/lib.rs
// also hasn’t changed, nor does use have any impact on what files are compiled
// as part of the crate. The mod keyword declares modules, and Rust looks in
// a file with the same name as the module for the code that goes into that module.
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
