#![allow(dead_code)]
#![allow(unused_imports)]

// It might seem like the paths we’ve written to call functions
// so far are inconveniently long and repetitive.
// Fortunately, there’s a way to simplify this process.
// We can bring a path into a scope once and then call the items
// in that path as if they’re local items with the use keyword.
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// Adding use and a path in a scope is similar to creating a symbolic
// link in the filesystem. By adding use crate::front_of_house::hosting
// in the crate root, hosting is now a valid name in that scope, just as
// though the hosting module had been defined in the crate root.
// Paths brought into scope with use also check privacy, like any other paths.
use crate::front_of_house::hosting;

// You can also bring an item into scope with use and a relative path.
//use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// Creating Idiomatic use Paths
// Specifying the parent module when calling the function makes it
// clear that the function isn’t locally defined while still minimizing
// repetition of the full path.
// This code in is unclear as to where add_to_waitlist is defined.
// use crate::front_of_house::hosting::add_to_waitlist;

// On the other hand, when bringing in structs, enums, and other items with use,
// it’s idiomatic to specify the full path.
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// The exception to this idiom is if we’re bringing two items with the
// same name into scope with use statements, because Rust doesn’t allow that.
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    Ok(())
}

fn function2() -> io::Result<()> {
    Ok(())
}

// Providing New Names with the as Keyword
// There’s another solution to the problem of bringing two types of
// the same name into the same scope with use: after the path,
// we can specify as and a new local name, or alias, for the type.
use std::fmt::Result;
use std::io::Result as IoResult;

fn function3() -> Result {
    Ok(())
}

fn function4() -> IoResult<()> {
    Ok(())
}


// When we bring a name into scope with the use keyword, the name available
// in the new scope is private. To enable the code that calls our code to
// refer to that name as if it had been defined in that code’s scope,
// we can combine pub and use. This technique is called re-exporting
// because we’re bringing an item into scope but also making that item
// available for others to bring into their scope.
//pub use crate::front_of_house::hosting;

// By using pub use, external code can now call the add_to_waitlist
// function using hosting::add_to_waitlist. If we hadn’t specified pub use,
// the eat_at_restaurant function could call hosting::add_to_waitlist
// in its scope, but external code couldn’t take advantage of this new path.

// Using External Packages
// We programmed a guessing game project that used an external package
// called rand to get random numbers. To use rand in our project,
// we added this line to Cargo.toml.
// rand = "0.8.3"
use rand::Rng;

fn main2() {
    let _secret_number = rand::thread_rng().gen_range(1..101);
}

// Members of the Rust community have made many packages available at crates.io,
// and pulling any of them into your package involves these same steps:
// listing them in your package’s Cargo.toml file and using use to bring
// items from their crates into scope.

// Using Nested Paths to Clean Up Large use Lists
// If we’re using multiple items defined in the same crate or same module,
// listing each item on its own line can take up a lot of vertical space in our files.
// Instead, we can use nested paths to bring the same items into scope in one line.
// We do this by specifying the common part of the path, followed by two colons,
// and then curly brackets around a list of the parts of the paths that differ.
//use std::{cmp::Ordering, io};

// We can use a nested path at any level in a path, which is useful
// when combining two use statements that share a subpath.
//use std::io::{self, Write};

// The Glob Operator
// If we want to bring all public items defined in a path into scope,
// we can specify that path followed by *.
//use std::collections::*;
