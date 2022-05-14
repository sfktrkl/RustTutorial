// Using a semicolon after mod front_of_house rather than using a block tells
// Rust to load the contents of the module from another file with the
// same name as the module. To continue with our example and extract the hosting
// module to its own file as well, we change src/front_of_house.rs to contain
// only the declaration of the hosting module
pub mod hosting;
