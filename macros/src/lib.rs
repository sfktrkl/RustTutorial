// Macros
// We’ve used macros like println! throughout this book, but we haven’t fully
// explored what a macro is and how it works. The term macro refers to a family
// of features in Rust: declarative macros with macro_rules! and three kinds of
// procedural macros:
//  Custom #[derive] macros that specify code added with the derive attribute
//      used on structs and enums
//  Attribute-like macros that define custom attributes usable on any item
//  Function-like macros that look like function calls but operate on the tokens
//      specified as their argument

// The Difference Between Macros and Functions
// Fundamentally, macros are a way of writing code that writes other code, which
// is known as metaprogramming. In Appendix C, we discuss the derive attribute,
// which generates an implementation of various traits for you. We’ve also used
// the println! and vec! macros throughout the book. All of these macros expand
// to produce more code than the code you’ve written manually.

// Metaprogramming is useful for reducing the amount of code you have to write
// and maintain, which is also one of the roles of functions. However, macros
// have some additional powers that functions don’t.

// A function signature must declare the number and type of parameters the
// function has. Macros, on the other hand, can take a variable number of
// parameters: we can call println!("hello") with one argument or
// println!("hello {}", name) with two arguments. Also, macros are expanded
// before the compiler interprets the meaning of the code, so a macro can, for
// example, implement a trait on a given type. A function can’t, because it gets
// called at runtime and a trait needs to be implemented at compile time.

// The downside to implementing a macro instead of a function is that macro
// definitions are more complex than function definitions because you’re writing
// Rust code that writes Rust code. Due to this indirection, macro definitions
// are generally more difficult to read, understand, and maintain than function
// definitions.

// Another important difference between macros and functions is that you must
// define macros or bring them into scope before you call them in a file, as
// opposed to functions you can define anywhere and call anywhere.

// Declarative Macros with macro_rules! for General Metaprogramming
// The most widely used form of macros in Rust is declarative macros. These are
// also sometimes referred to as “macros by example,” “macro_rules! macros,” or
// just plain “macros.” At their core, declarative macros allow you to write
// something similar to a Rust match expression. Match expressions are control
// structures that take an expression, compare the resulting value of the
// expression to patterns, and then run the code associated with the matching
// pattern. Macros also compare a value to patterns that are associated with
// particular code: in this situation, the value is the literal Rust source code
// passed to the macro; the patterns are compared with the structure of that
// source code; and the code associated with each pattern, when matched,
// replaces the code passed to the macro. This all happens during compilation.

// To define a macro, you use the macro_rules! construct. Let’s explore how to
// use macro_rules! by looking at how the vec! macro is defined.

// We could also use the vec! macro to make a vector of two integers or a vector
// of five string slices. We wouldn’t be able to use a function to do the same
// because we wouldn’t know the number or type of values up front.

// Note: The actual definition of the vec! macro in the standard library
// includes code to preallocate the correct amount of memory up front. That code
// is an optimization that we don’t include here to make the example simpler.

// The #[macro_export] annotation indicates that this macro should be made
// available whenever the crate in which the macro is defined is brought into
// scope. Without this annotation, the macro can’t be brought into scope.
#[macro_export]
// We then start the macro definition with macro_rules! and the name of the
// macro we’re defining without the exclamation mark. The name, in this case
// vec, is followed by curly brackets denoting the body of the macro definition.
macro_rules! vec {
    // The structure in the vec! body is similar to the structure of a match
    // expression. Here we have one arm with the pattern ( $( $x:expr ),* ),
    // followed by => and the block of code associated with this pattern. If the
    // pattern matches, the associated block of code will be emitted. Given that
    // this is the only pattern in this macro, there is only one valid way to
    // match; any other pattern will result in an error. More complex macros
    // will have more than one arm.

    // First, a set of parentheses encompasses the whole pattern. A dollar sign
    // ($) is next, followed by a set of parentheses that captures values that
    // match the pattern within the parentheses for use in the replacement code.
    // Within $() is $x:expr, which matches any Rust expression and gives the
    // expression the name $x.

    // The comma following $() indicates that a literal comma separator
    // character could optionally appear after the code that matches the code in
    // $(). The * specifies that the pattern matches zero or more of whatever
    // precedes the *.
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            // Now let’s look at the pattern in the body of the code associated
            // with this arm: temp_vec.push() within $()* is generated for each
            // part that matches $() in the pattern zero or more times depending
            // on how many times the pattern matches. The $x is replaced with
            // each expression matched.
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// Procedural Macros for Generating Code from Attributes
// The second form of macros is procedural macros, which act more like functions
// (and are a type of procedure). Procedural macros accept some code as an
// input, operate on that code, and produce some code as an output rather than
// matching against patterns and replacing the code with other code as
// declarative macros do.

// The three kinds of procedural macros (custom derive, attribute-like, and
// function-like) all work in a similar fashion.

// When creating procedural macros, the definitions must reside in their own
// crate with a special crate type. This is for complex technical reasons that
// we hope to eliminate in the future.
