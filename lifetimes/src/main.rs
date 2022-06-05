#![allow(dead_code)]
use std::fmt::Display;

fn main() {
    // Validating References with Lifetimes
    // Every reference in Rust has a lifetime, which is the scope for
    // which that reference is valid. Most of the time, lifetimes are implicit
    // and inferred, just like most of the time, types are inferred.
    // We only must annotate types when multiple types are possible.
    // In a similar way, we must annotate lifetimes when the lifetimes
    // of references could be related in a few different ways. Rust requires
    // us to annotate the relationships using generic lifetime parameters to
    // ensure the actual references used at runtime will definitely be valid.

    // Preventing Dangling References with Lifetimes
    // The main aim of lifetimes is to prevent dangling references,
    // which cause a program to reference data other than the data it’s
    // intended to reference.

    // The Borrow Checker
    // The Rust compiler has a borrow checker that compares scopes to
    // determine whether all borrows are valid.
    // Here, x has the lifetime 'b, which in this case is larger than 'a.
    // This means r can reference x because Rust knows that the reference in
    // r will always be valid while x is valid.
    {
        let x = 5; // ----------+-- 'b
                   //           |
        let r = &x; // --+-- 'a  |
                    //   |       |
        println!("r: {}", r); //   |       |
                              // --+       |
    } // ----------+

    // Generic Lifetimes in Functions
    // We’ll write a function that returns the longer of two string slices.
    // This function will take two string slices and return a single string slice.
    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // Lifetime Annotation Syntax
    // Lifetime annotations don’t change how long any of the references live.
    // Rather, they describe the relationships of the lifetimes of multiple
    // references to each other without affecting the lifetimes. Just as
    // functions can accept any type when the signature specifies a generic
    // type parameter, functions can accept references with any lifetime by
    // specifying a generic lifetime parameter.
    //&i32        // a reference
    //&'a i32     // a reference with an explicit lifetime
    //&'a mut i32 // a mutable reference with an explicit lifetime

    // In this example, string1 is valid until the end of the outer scope,
    // string2 is valid until the end of the inner scope, and result references
    // something that is valid until the end of the inner scope. Run this code,
    // and you’ll see that the borrow checker approves; it will compile and print
    // The longest string is long string is long.
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // Let’s try an example that shows that the lifetime of the reference
    // in result must be the smaller lifetime of the two arguments.
    // The error shows that for result to be valid for the println! statement,
    // string2 would need to be valid until the end of the outer scope. Rust
    // knows this because we annotated the lifetimes of the function parameters
    // and return values using the same lifetime parameter 'a.
    /*
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
    */

    // This struct has one field, part, that holds a string slice, which is
    // a reference. As with generic data types, we declare the name of the
    // generic lifetime parameter inside angle brackets after the name of the
    // struct so we can use the lifetime parameter in the body of the struct
    // definition. This annotation means an instance of ImportantExcerpt can’t
    // outlive the reference it holds in its part field.
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let _i = ImportantExcerpt {
        part: first_sentence,
    };

    // The Static Lifetime
    // One special lifetime we need to discuss is 'static, which denotes that
    // the affected reference can live for the entire duration of the program.
    // All string literals have the 'static lifetime.
    // The text of this string is stored directly in the program’s binary,
    // which is always available. Therefore, the lifetime of all string literals
    // is 'static.
    let _s: &'static str = "I have a static lifetime.";
}

// Rust can’t tell whether the reference being returned refers to x or y.
// Actually, we don’t know either, because the if block in the body of this
// function returns a reference to x and the else block returns a reference to y!
// The borrow checker can’t determine this either, because it doesn’t know how
// the lifetimes of x and y relate to the lifetime of the return value. To fix
// this error, we’ll add generic lifetime parameters that define the relationship
// between the references so the borrow checker can perform its analysis.
// The function signature now tells Rust that for some lifetime 'a, the function
// takes two parameters, both of which are string slices that live at least
// as long as lifetime 'a. The function signature also tells Rust that the
// string slice returned from the function will live at least as long as
// lifetime 'a. In practice, it means that the lifetime of the reference returned
// by the longest function is the same as the smaller of the lifetimes of the
// references passed in. These relationships are what we want Rust to use when
// analyzing this code.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Thinking in Terms of Lifetimes
// The way in which you need to specify lifetime parameters depends on what
// your function is doing. For example, if we changed the implementation of
// the longest function to always return the first parameter rather than the
// longest string slice, we wouldn’t need to specify a lifetime on the y parameter.
fn longest2<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

// When returning a reference from a function, the lifetime parameter for the
// return type needs to match the lifetime parameter for one of the parameters.
// If the reference returned does not refer to one of the parameters, it must
// refer to a value created within this function. However, this would be
// a dangling reference because the value will go out of scope at the end of
// the function.
// The problem is that result goes out of scope and gets cleaned up at the
// end of the longest function. We’re also trying to return a reference to
// result from the function. There is no way we can specify lifetime parameters
// that would change the dangling reference, and Rust won’t let us create
// a dangling reference.
/*
fn longest3<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
*/

// Lifetime Annotations in Struct Definitions
// So far, the structs we've define all hold owned types. We can define structs
// to hold references, but in that case we would need to add a lifetime
// annotation on every reference in the struct’s definition.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Lifetime Elision
// You’ve learned that every reference has a lifetime and that you need to
// specify lifetime parameters for functions or structs that use references.
//fn first_word<'a>(s: &'a str) -> &'a str {

// After writing a lot of Rust code, the Rust team found that Rust programmers
// were entering the same lifetime annotations over and over in particular
// situations. These situations were predictable and followed a few deterministic
// patterns. The developers programmed these patterns into the compiler’s code
// so the borrow checker could infer the lifetimes in these situations and
// wouldn’t need explicit annotations.

// The patterns programmed into Rust’s analysis of references are called the
// lifetime elision rules. These aren’t rules for programmers to follow;
// they’re a set of particular cases that the compiler will consider,
// and if your code fits these cases, you don’t need to write the lifetimes
// explicitly.

// Lifetimes on function or method parameters are called input lifetimes,
// and lifetimes on return values are called output lifetimes.

// The compiler uses three rules to figure out the lifetimes of the references
// when there aren’t explicit annotations. The first rule applies to input
// lifetimes, and the second and third rules apply to output lifetimes.
// If the compiler gets to the end of the three rules and there are still
// references for which it can’t figure out lifetimes, the compiler will
// stop with an error. These rules apply to fn definitions as well as impl blocks.
//fn first_word(s: &str) -> &str {
//fn longest(x: &str, y: &str) -> &str {

// The first rule is that the compiler assigns a lifetime parameter to each
// parameter that’s a reference.
//fn first_word<'a>(s: &'a str) -> &str {
//fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
// The second rule is that, if there is exactly one input lifetime parameter,
// that lifetime is assigned to all output lifetime parameters.
//fn first_word<'a>(s: &'a str) -> &'a str {
//fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str { // second rule doesn't apply
// The third rule is that, if there are multiple input lifetime parameters,
// but one of them is &self or &mut self because this is a method, the lifetime
// of self is assigned to all output lifetime parameters. This third rule makes
// methods much nicer to read and write because fewer symbols are necessary.

// Lifetime Annotations in Method Definitions
// Lifetime names for struct fields always need to be declared after the impl
// keyword and then used after the struct’s name, because those lifetimes are
// part of the struct’s type.

// First, we’ll use a method named level whose only parameter is a reference
// to self and whose return value is an i32, which is not a reference to anything.
// The lifetime parameter declaration after impl and its use after the type name
// are required, but we’re not required to annotate the lifetime of the reference
// to self because of the first elision rule.
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}

// Here is an example where the third lifetime elision rule applies
// There are two input lifetimes, so Rust applies the first lifetime elision
// rule and gives both &self and announcement their own lifetimes. Then,
// because one of the parameters is &self, the return type gets the lifetime
// of &self, and all lifetimes have been accounted for.
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// Generic Type Parameters, Trait Bounds, and Lifetimes Together
// Let’s briefly look at the syntax of specifying generic type parameters,
// trait bounds, and lifetimes all in one function!
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
