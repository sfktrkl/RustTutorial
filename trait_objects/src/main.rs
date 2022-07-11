use trait_objects::Draw;
use trait_objects::{Button, Screen};

// If someone using our library decides to implement a SelectBox struct that has
// width, height, and options fields, they implement the Draw trait on the
// SelectBox type as well.
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "SelectBox w:{}, h:{}, options:{:?}",
            self.width, self.height, self.options
        );
    }
}

fn main() {
    // Our library’s user can now write their main function to create a Screen
    // instance. To the Screen instance, they can add a SelectBox and a Button
    // by putting each in a Box<T> to become a trait object. They can then call
    // the run method on the Screen instance, which will call draw on each of
    // the components.
    let screen = Screen {
        // When we wrote the library, we didn’t know that someone might add the
        // SelectBox type, but our Screen implementation was able to operate on
        // the new type and draw it because SelectBox implements the Draw trait,
        // which means it implements the draw method.
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    // This concept—of being concerned only with the messages a value responds
    // to rather than the value’s concrete type—is similar to the concept of
    // duck typing in dynamically typed languages: if it walks like a duck and
    // quacks like a duck, then it must be a duck! In the implementation of run
    // on Screen, run doesn’t need to know what the concrete type of each
    // component is. It doesn’t check whether a component is an instance of a
    // Button or a SelectBox, it just calls the draw method on the component. By
    // specifying Box<dyn Draw> as the type of the values in the components
    // vector, we’ve defined Screen to need values that we can call the draw
    // method on.

    // The advantage of using trait objects and Rust’s type system to write code
    // similar to code using duck typing is that we never have to check whether
    // a value implements a particular method at runtime or worry about getting
    // errors if a value doesn’t implement a method but we call it anyway. Rust
    // won’t compile our code if the values don’t implement the traits that the
    // trait objects need.
    screen.run();

    // Trait Objects Perform Dynamic Dispatch
    // Recall our discussion on the monomorphization process performed by the
    // compiler when we use trait bounds on generics: the compiler generates
    // nongeneric implementations of functions and methods for each concrete
    // type that we use in place of a generic type parameter. The code that
    // results from monomorphization is doing static dispatch, which is when the
    // compiler knows what method you’re calling at compile time. This is
    // opposed to dynamic dispatch, which is when the compiler can’t tell at
    // compile time which method you’re calling. In dynamic dispatch cases, the
    // compiler emits code that at runtime will figure out which method to call.

    // When we use trait objects, Rust must use dynamic dispatch. The compiler
    // doesn’t know all the types that might be used with the code that is using
    // trait objects, so it doesn’t know which method implemented on which type
    // to call. Instead, at runtime, Rust uses the pointers inside the trait
    // object to know which method to call. There is a runtime cost when this
    // lookup happens that doesn’t occur with static dispatch. Dynamic dispatch
    // also prevents the compiler from choosing to inline a method’s code, which
    // in turn prevents some optimizations.
}
