// Running Code on Cleanup with the Drop Trait
// The second trait important to the smart pointer pattern is Drop, which lets
// you customize what happens when a value is about to go out of scope. You can
// provide an implementation for the Drop trait on any type, and the code you
// specify can be used to release resources like files or network connections.
// We’re introducing Drop in the context of smart pointers because the
// functionality of the Drop trait is almost always used when implementing
// a smart pointer. For example, when a Box<T> is dropped it will deallocate
// the space on the heap that the box points to.

// In some languages, the programmer must call code to free memory or resources
// every time they finish using an instance of a smart pointer. If they forget,
// the system might become overloaded and crash. In Rust, you can specify that
// a particular bit of code be run whenever a value goes out of scope, and the
// compiler will insert this code automatically. As a result, you don’t need
// to be careful about placing cleanup code everywhere in a program that an
// instance of a particular type is finished with—you still won’t leak resources!

// Specify the code to run when a value goes out of scope by implementing the
// Drop trait. The Drop trait requires you to implement one method named drop
// that takes a mutable reference to self. To see when Rust calls drop, let’s
// implement drop with println! statements for now.
struct CustomSmartPointer {
    data: String,
}

// The Drop trait is included in the prelude, so we don’t need to bring it into
// scope. We implement the Drop trait on CustomSmartPointer and provide an
// implementation for the drop method that calls println!. The body of the drop
// function is where you would place any logic that you wanted to run when an
// instance of your type goes out of scope. We’re printing some text here to
// demonstrate when Rust will call drop.
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    // n main, we create two instances of CustomSmartPointer and then print
    // CustomSmartPointers created. At the end of main, our instances of
    // CustomSmartPointer will go out of scope, and Rust will call the code
    // we put in the drop method, printing our final message. Note that we
    // didn’t need to call the drop method explicitly.
    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // Dropping a Value Early with std::mem::drop
    // Unfortunately, it’s not straightforward to disable the automatic drop
    // functionality. Disabling drop isn’t usually necessary; the whole point
    // of the Drop trait is that it’s taken care of automatically. Occasionally,
    // however, you might want to clean up a value early. One example is when
    // using smart pointers that manage locks: you might want to force the drop
    // method that releases the lock so that other code in the same scope can
    // acquire the lock. Rust doesn’t let you call the Drop trait’s drop method
    // manually; instead you have to call the std::mem::drop function provided
    // by the standard library if you want to force a value to be dropped before
    // the end of its scope.
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

    // You can use code specified in a Drop trait implementation in many ways
    // to make cleanup convenient and safe: for instance, you could use it to
    // create your own memory allocator! With the Drop trait and Rust’s
    // ownership system, you don’t have to remember to clean up because Rust
    // does it automatically.

    // You also don’t have to worry about problems resulting from accidentally
    // cleaning up values still in use: the ownership system that makes sure
    // references are always valid also ensures that drop gets called only
    // once when the value is no longer being used.
}
