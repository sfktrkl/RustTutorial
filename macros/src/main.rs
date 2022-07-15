// How to Write a Custom derive Macro
// Let’s create a crate named hello_macro that defines a trait named HelloMacro
// with one associated function named hello_macro. Rather than making our crate
// users implement the HelloMacro trait for each of their types, we’ll provide a
// procedural macro so users can annotate their type with #[derive(HelloMacro)]
// to get a default implementation of the hello_macro function. The default
// implementation will print Hello, Macro! My name is TypeName! where TypeName
// is the name of the type on which this trait has been defined.

// However, they would need to write the implementation block for each type they
// wanted to use with hello_macro; we want to spare them from having to do this
// work.

// Additionally, we can’t yet provide the hello_macro function with default
// implementation that will print the name of the type the trait is implemented
// on: Rust doesn’t have reflection capabilities, so it can’t look up the type’s
// name at runtime. We need a macro to generate code at compile time.

// The next step is to define the procedural macro. At the time of this writing,
// procedural macros need to be in their own crate. Eventually, this restriction
// might be lifted. The convention for structuring crates and macro crates is as
// follows: for a crate named foo, a custom derive procedural macro crate is
// called foo_derive. Let’s start a new crate called hello_macro_derive inside
// our hello_macro project.

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}

// Attribute-like macros
// Attribute-like macros are similar to custom derive macros, but instead of
// generating code for the derive attribute, they allow you to create new
// attributes. They’re also more flexible: derive only works for structs and
// enums; attributes can be applied to other items as well, such as functions.
// Here’s an example of using an attribute-like macro: say you have an attribute
// named route that annotates functions when using a web application framework:
//#[route(GET, "/")]
//fn index() {}

// This #[route] attribute would be defined by the framework as a procedural
// macro. The signature of the macro definition function would look like this:
//#[proc_macro_attribute]
//pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}

// Here, we have two parameters of type TokenStream. The first is for the
// contents of the attribute: the GET, "/" part. The second is the body of the
// item the attribute is attached to: in this case, fn index() {} and the rest
// of the function’s body.

// Other than that, attribute-like macros work the same way as custom derive
// macros: you create a crate with the proc-macro crate type and implement a
// function that generates the code you want!

// Function-like macros
// Function-like macros define macros that look like function calls. Similarly
// to macro_rules! macros, they’re more flexible than functions; for example,
// they can take an unknown number of arguments. However, macro_rules! macros
// can be defined only using the match-like syntax we discussed in the section
// “Declarative Macros with macro_rules! for General Metaprogramming” earlier.
// Function-like macros take a TokenStream parameter and their definition
// manipulates that TokenStream using Rust code as the other two types of
// procedural macros do. An example of a function-like macro is an sql! macro
// that might be called like so:
//let sql = sql!(SELECT * FROM posts WHERE id=1);

// This macro would parse the SQL statement inside it and check that it’s
// syntactically correct, which is much more complex processing than a
// macro_rules! macro can do. The sql! macro would be defined like this:
//#[proc_macro]
//pub fn sql(input: TokenStream) -> TokenStream {
