// Another style of doc comment, //!, adds documentation to the item that contains
// the comments rather than adding documentation to the items following the comments.
// We typically use these doc comments inside the crate root file (src/lib.rs
// by convention) or inside a module to document the crate or the module as a whole.
//! # Add One Crate
//!
//! `add_one` is a collection of utilities to make performing certain
//! calculations more convenient.

// Publishing a Crate to Crates.io
// We’ve used packages from crates.io as dependencies of our project, but you
// can also share your code with other people by publishing your own packages.
// The crate registry at crates.io distributes the source code of your packages,
// so it primarily hosts code that is open source.

// Making Useful Documentation Comments
// Accurately documenting your packages will help other users know how and when
// to use them, so it’s worth investing the time to write documentation.
// Rust also has a particular kind of comment for documentation, known
// conveniently as a documentation comment, that will generate HTML
// documentation. The HTML displays the contents of documentation comments
// for public API items intended for programmers interested in knowing how to
// use your crate as opposed to how your crate is implemented.
// Documentation comments use three slashes, ///, instead of two and support
// Markdown notation for formatting the text.

// We can generate the HTML documentation from this documentation comment by
// running cargo doc. This command runs the rustdoc tool distributed with Rust
// and puts the generated HTML documentation in the target/doc directory.

// For convenience, running cargo doc --open will build the HTML for your current
// crate’s documentation (as well as the documentation for all of your crate’s
// dependencies) and open the result in a web browser.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = add_one::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// Commonly Used Sections
// We used the # Examples Markdown heading to create a section in the HTML with
// the title “Examples.” Here are some other sections that crate authors commonly
// use in their documentation: 
//  Panics: The scenarios in which the function being documented could panic.
//      Callers of the function who don’t want their programs to panic should make
//      sure they don’t call the function in these situations.
//  Errors: If the function returns a Result, describing the kinds of errors
//      that might occur and what conditions might cause those errors to be
//      returned can be helpful to callers so they can write code to handle
//      the different kinds of errors in different ways.
//  Safety: If the function is unsafe to call, there should be a section 
//      explaining why the function is unsafe and covering the invariants that
//      the function expects callers to uphold.

// Documentation Comments as Tests
// Adding example code blocks in your documentation comments can help demonstrate
// how to use your library, and doing so has an additional bonus: running cargo
// test will run the code examples in your documentation as tests! Nothing is
// better than documentation with examples. But nothing is worse than examples
// that don’t work because the code has changed since the documentation was written.

// we covered how to organize our code into modules using the mod keyword, how
// to make items public using the pub keyword, and how to bring items into a
// scope with the use keyword. However, the structure that makes sense to you
// while you’re developing a crate might not be very convenient for your users.
// You might want to organize your structs in a hierarchy containing multiple
// levels, but then people who want to use a type you’ve defined deep in the
// hierarchy might have trouble finding out that type exists. They might also
// be annoyed at having to enter use my_crate::some_module::another_module::UsefulType;
// rather than use my_crate::UsefulType;.

// The structure of your public API is a major consideration when publishing
// a crate. People who use your crate are less familiar with the structure than
// you are and might have difficulty finding the pieces they want to use if your
// crate has a large module hierarchy.

// The good news is that if the structure isn’t convenient for others to use
// from another library, you don’t have to rearrange your internal organization:
// instead, you can re-export items to make a public structure that’s different
// from your private structure by using pub use. Re-exporting takes a public
// item in one location and makes it public in another location, as if it were
// defined in the other location instead.
