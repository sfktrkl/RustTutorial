// The author of the code, which uses the art crate, had to figure out that
// PrimaryColor is in the kinds module and mix is in the utils module. The
// module structure of the art crate is more relevant to developers working
// on the art crate than to developers using the art crate. The internal
// structure that organizes parts of the crate into the kinds module and the
// utils module doesn’t contain any useful information for someone trying to
// understand how to use the art crate. Instead, the art crate’s module
// structure causes confusion because developers have to figure out where
// to look, and the structure is inconvenient because developers must specify
// the module names in the use statements.
use art::kinds::PrimaryColor;
use art::utils::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
