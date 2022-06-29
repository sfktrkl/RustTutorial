#![allow(dead_code)]
// Using Box<T> to Point to Data on the Heap
// The most straightforward smart pointer is a box, whose type is written Box<T>.
// Boxes allow you to store data on the heap rather than the stack.

// Boxes don’t have performance overhead, other than storing their data on the
// heap instead of on the stack. But they don’t have many extra capabilities
// either. You’ll use them most often in these situations:
//  When you have a type whose size can’t be known at compile time and you want
//      to use a value of that type in a context that requires an exact size
//  When you have a large amount of data and you want to transfer ownership
//      but ensure the data won’t be copied when you do so
//  When you want to own a value and you care only that it’s a type that
//      implements a particular trait rather than being of a specific type

use crate::List::{Cons, Nil};
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // Using a Box<T> to Store Data on the Heap
    // Before we discuss this use case for Box<T>, we’ll cover the syntax and
    // how to interact with values stored within a Box<T>.
    let b = Box::new(5);
    // We define the variable b to have the value of a Box that points to the
    // value 5, which is allocated on the heap. This program will print b = 5;
    // in this case, we can access the data in the box similar to how we would
    // if this data were on the stack. Just like any owned value, when a box
    // goes out of scope, as b does at the end of main, it will be deallocated.
    // The deallocation happens for the box (stored on the stack) and the data
    // it points to (stored on the heap).
    println!("b = {}", b);

    // Enabling Recursive Types with Boxes
    // At compile time, Rust needs to know how much space a type takes up. One
    // type whose size can’t be known at compile time is a recursive type, where
    // a value can have as part of itself another value of the same type. Because
    // this nesting of values could theoretically continue infinitely, Rust
    // doesn’t know how much space a value of a recursive type needs. However,
    // boxes have a known size, so by inserting a box in a recursive type
    // definition, you can have recursive types.
    // Let’s explore the cons list, which is a data type common in functional
    // programming languages, as an example of a recursive type. The cons list
    // type we’ll define is straightforward except for the recursion; therefore,
    // the concepts in the example we’ll work with will be useful any time you
    // get into more complex situations involving recursive types.

    // More Information About the Cons List
    // A cons list is a data structure that comes from the Lisp programming
    // language and its dialects. In Lisp, the cons function (short for
    // “construct function”) constructs a new pair from its two arguments,
    // which usually are a single value and another pair. These pairs containing
    // pairs form a list.
    // The cons function concept has made its way into more general functional
    // programming jargon: “to cons x onto y” informally means to construct
    // a new container instance by putting the element x at the start of this
    // new container, followed by the container y.
    // Each item in a cons list contains two elements: the value of the current
    // item and the next item. The last item in the list contains only a value
    // called Nil without a next item. A cons list is produced by recursively
    // calling the cons function. The canonical name to denote the base case
    // of the recursion is Nil.

    // Although functional programming languages use cons lists frequently,
    // the cons list isn’t a commonly used data structure in Rust. Most of the
    // time when you have a list of items in Rust, Vec<T> is a better choice to
    // use. Other, more complex recursive data types are useful in various
    // situations, but by starting with the cons list, we can explore how boxes
    // let us define a recursive data type without much distraction.

    // Using Box<T> to Get a Recursive Type with a Known Size
    // Rust can’t figure out how much space to allocate for recursively defined types.
    // In this suggestion, “indirection” means that instead of storing a value
    // directly, we’ll change the data structure to store the value indirectly
    // by storing a pointer to the value instead.
    // Because a Box<T> is a pointer, Rust always knows how much space a Box<T>
    // needs: a pointer’s size doesn’t change based on the amount of data it’s
    // pointing to. This means we can put a Box<T> inside the Cons variant
    // instead of another List value directly. The Box<T> will point to the
    // next List value that will be on the heap rather than inside the Cons
    // variant. Conceptually, we still have a list, created with lists “holding”
    // other lists, but this implementation is now more like placing the items
    // next to one another rather than inside one another.
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // The Cons variant will need the size of an i32 plus the space to store the
    // box’s pointer data. The Nil variant stores no values, so it needs less
    // space than the Cons variant. We now know that any List value will take
    // up the size of an i32 plus the size of a box’s pointer data. By using
    // a box, we’ve broken the infinite, recursive chain, so the compiler can
    // figure out the size it needs to store a List value.

    // The Box<T> type is a smart pointer because it implements the Deref trait,
    // which allows Box<T> values to be treated like references. When a Box<T>
    // value goes out of scope, the heap data that the box is pointing to is
    // cleaned up as well because of the Drop trait implementation.
}

// Computing the Size of a Non-Recursive Type
// To determine how much space to allocate for a Message value, Rust goes
// through each of the variants to see which variant needs the most space.
// Rust sees that Message::Quit doesn’t need any space, Message::Move needs
// enough space to store two i32 values, and so forth. Because only one variant
// will be used, the most space a Message value will need is the space it would
// take to store the largest of its variants.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Contrast this with what happens when Rust tries to determine how much space
// a recursive type like the List. The compiler starts by looking at the Cons
// variant, which holds a value of type i32 and a value of type List. Therefore,
// Cons needs an amount of space equal to the size of an i32 plus the size of
// a List. To figure out how much memory the List type needs, the compiler looks
// at the variants, starting with the Cons variant. The Cons variant holds a
// value of type i32 and a value of type List, and this process continues infinitely.
enum List2 {
    Cons(i32, List),
    Nil,
}
