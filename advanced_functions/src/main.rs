#![allow(dead_code)]

// Function Pointers
// We’ve talked about how to pass closures to functions; you can also pass
// regular functions to functions! This technique is useful when you want to
// pass a function you’ve already defined rather than defining a new
// closure. Doing this with function pointers will allow you to use
// functions as arguments to other functions. Functions coerce to the type
// fn (with a lowercase f), not to be confused with the Fn closure trait.
// The fn type is called a function pointer. The syntax for specifying that
// a parameter is a function pointer is similar to that of closures.
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    // This code prints The answer is: 12. We specify that the parameter f in
    // do_twice is an fn that takes one parameter of type i32 and returns an
    // i32. We can then call f in the body of do_twice. In main, we can pass the
    // function name add_one as the first argument to do_twice.
    let answer = do_twice(add_one, 5);
    println!("The answer is: {}", answer);

    // Unlike closures, fn is a type rather than a trait, so we specify fn as
    // the parameter type directly rather than declaring a generic type
    // parameter with one of the Fn traits as a trait bound.

    // Function pointers implement all three of the closure traits (Fn, FnMut,
    // and FnOnce), so you can always pass a function pointer as an argument for
    // a function that expects a closure. It’s best to write functions using a
    // generic type and one of the closure traits so your functions can accept
    // either functions or closures.

    // An example of where you would want to only accept fn and not closures is
    // when interfacing with external code that doesn’t have closures: C
    // functions can accept functions as arguments, but C doesn’t have closures.

    // As an example of where you could use either a closure defined inline or a
    // named function, let’s look at a use of map. To use the map function to
    // turn a vector of numbers into a vector of strings, we could use a
    // closure, like this:
    let list_of_numbers = vec![1, 2, 3];
    let _list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    // Or we could name a function as the argument to map instead of the
    // closure, like this:
    let list_of_numbers = vec![1, 2, 3];
    let _list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    // Note that we must use the fully qualified syntax that we talked about
    // earlier in the “Advanced Traits” section because there are multiple
    // functions available named to_string. Here, we’re using the to_string
    // function defined in the ToString trait, which the standard library has
    // implemented for any type that implements Display.

    // We can use these initializer functions as function pointers that
    // implement the closure traits, which means we can specify the initializer
    // functions as arguments for methods that take closures, like so:
    enum Status {
        Value(u32),
        Stop,
    }

    // Here we create Status::Value instances using each u32 value in the range
    // that map is called on by using the initializer function of Status::Value.
    // Some people prefer this style, and some people prefer to use closures.
    // They compile to the same code, so use whichever style is clearer to you.
    let _list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    // Returning Closures
    // Closures are represented by traits, which means you can’t return closures
    // directly. In most cases where you might want to return a trait, you can
    // instead use the concrete type that implements the trait as the return
    // value of the function. But you can’t do that with closures because they
    // don’t have a concrete type that is returnable; you’re not allowed to use
    // the function pointer fn as a return type, for example.
    /*
    fn returns_closure() -> dyn Fn(i32) -> i32 {
        |x| x + 1
    }
    */

    // The error references the Sized trait again! Rust doesn’t know how much
    // space it will need to store the closure. We saw a solution to this
    // problem earlier. We can use a trait object:
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
}
