use std::ops::Deref;
// Treating Smart Pointers Like Regular References with the Deref Trait
// Implementing the Deref trait allows you to customize the behavior of the
// dereference operator, * (as opposed to the multiplication or glob operator).
// By implementing Deref in such a way that a smart pointer can be treated like
// a regular reference, you can write code that operates on references and use
// that code with smart pointers too.

fn main() {
    // Following the Pointer to the Value with the Dereference Operator
    // A regular reference is a type of pointer, and one way to think of
    // a pointer is as an arrow to a value stored somewhere else. We create
    // a reference to an i32 value and then use the dereference operator
    // to follow the reference to the data:
    let x = 5;
    let y = &x;
    // The variable x holds an i32 value, 5. We set y equal to a reference to x.
    // We can assert that x is equal to 5. However, if we want to make
    // an assertion about the value in y, we have to use *y to follow the
    // reference to the value it’s pointing to (hence dereference). Once we
    // dereference y, we have access to the integer value y is pointing to
    // that we can compare with 5.
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);
    // Our MyBox<T> type can’t be dereferenced because we haven’t implemented
    // that ability on our type. To enable dereferencing with the * operator,
    // we implement the Deref trait.
    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Implicit Deref Coercions with Functions and Methods
    // Deref coercion is a convenience that Rust performs on arguments to
    // functions and methods. Deref coercion works only on types that implement
    // the Deref trait. Deref coercion converts a reference to such a type into
    // a reference to another type. For example, deref coercion can convert
    // &String to &str because String implements the Deref trait such that it
    // returns &str. Deref coercion happens automatically when we pass
    // a reference to a particular type’s value as an argument to a function
    // or method that doesn’t match the parameter type in the function or
    // method definition. A sequence of calls to the deref method converts
    // the type we provided into the type the parameter needs.
    // Deref coercion was added to Rust so that programmers writing function
    // and method calls don’t need to add as many explicit references and
    // dereferences with & and *. The deref coercion feature also lets us
    // write more code that can work for either references or smart pointers.

    // We can call the hello function with a string slice as an argument, such
    // as hello("Rust"); for example. Deref coercion makes it possible to call
    // hello with a reference to a value of type MyBox<String>.
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // Here we’re calling the hello function with the argument &m, which is
    // a reference to a MyBox<String> value. Because we implemented the Deref
    // trait on MyBox<T>. Rust can turn &MyBox<String> into &String by calling
    // deref. The standard library provides an implementation of Deref on
    // String that returns a string slice, and this is in the API documentation
    // for Deref. Rust calls deref again to turn the &String into &str, which
    // matches the hello function’s definition.
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);

    // The (*m) dereferences the MyBox<String> into a String. Then the & and [..]
    // take a string slice of the String that is equal to the whole string to
    // match the signature of hello. The code without deref coercions is harder
    // to read, write, and understand with all of these symbols involved.
    // Deref coercion allows Rust to handle these conversions for us automatically.

    // When the Deref trait is defined for the types involved, Rust will analyze
    // the types and use Deref::deref as many times as necessary to get
    // a reference to match the parameter’s type. The number of times that
    // Deref::deref needs to be inserted is resolved at compile time, so there
    // is no runtime penalty for taking advantage of deref coercion!

    // How Deref Coercion Interacts with Mutability
    // Similar to how you use the Deref trait to override the * operator on
    // immutable references, you can use the DerefMut trait to override the *
    // operator on mutable references.

    // Rust does deref coercion when it finds types and trait
    // implementations in three cases:
    //      From &T to &U when T: Deref<Target=U>
    //      From &mut T to &mut U when T: DerefMut<Target=U>
    //      From &mut T to &U when T: Deref<Target=U>

    // The first two cases are the same except for mutability. The first case
    // states that if you have a &T, and T implements Deref to some type U,
    // you can get a &U transparently. The second case states that the same
    // deref coercion happens for mutable references.

    // The third case is trickier: Rust will also coerce a mutable reference
    // to an immutable one. But the reverse is not possible: immutable references
    // will never coerce to mutable references. Because of the borrowing rules,
    // if you have a mutable reference, that mutable reference must be the only
    // reference to that data (otherwise, the program wouldn’t compile).
    // Converting one mutable reference to one immutable reference will never
    // break the borrowing rules. Converting an immutable reference to a mutable
    // reference would require that the initial immutable reference is the only
    // immutable reference to that data, but the borrowing rules don’t guarantee
    // that. Therefore, Rust can’t make the assumption that converting
    // an immutable reference to a mutable reference is possible.
}

// Defining Our Own Smart Pointer
// Let’s build a smart pointer similar to the Box<T> type provided by the
// standard library to experience how smart pointers behave differently from
// references by default. Then we’ll look at how to add the ability to use
// the dereference operator.
struct MyBox<T>(T);
// We define a struct named MyBox and declare a generic parameter T, because we
// want our type to hold values of any type. The MyBox type is a tuple struct
// with one element of type T. The MyBox::new function takes one parameter of
// type T and returns a MyBox instance that holds the value passed in.
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
// The Deref trait, provided by the standard library, requires us to implement
// one method named deref that borrows self and returns a reference to the inner data.
impl<T> Deref for MyBox<T> {
    // The type Target = T; syntax defines an associated type for the Deref trait to use.
    type Target = T;
    // We fill in the body of the deref method with &self.0 so deref returns
    // a reference to the value we want to access with the * operator.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
    // Without the Deref trait, the compiler can only dereference & references.
    // The deref method gives the compiler the ability to take a value of any
    // type that implements Deref and call the deref method to get a & reference
    // that it knows how to dereference.

    // Rust substitutes the * operator with a call to the deref method and then
    // a plain dereference so we don’t have to think about whether or not we need
    // to call the deref method. This Rust feature lets us write code that
    // functions identically whether we have a regular reference or a type
    // that implements Deref.

    // The reason the deref method returns a reference to a value, and that
    // the plain dereference outside the parentheses in *(y.deref()) is still
    // necessary, is the ownership system. If the deref method returned the
    // value directly instead of a reference to the value, the value would
    // be moved out of self. We don’t want to take ownership of the inner
    // value inside MyBox<T> in this case or in most cases where we use
    // the dereference operator.
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
