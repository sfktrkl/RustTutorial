// Ownership Rules
// First, let’s take a look at the ownership rules.
// Keep these rules in mind as we work through the examples that illustrate them:
//    Each value in Rust has a variable that’s called its owner.
//    There can only be one owner at a time.
//    When the owner goes out of scope, the value will be dropped.
fn main() {
    {
        // s is not valid here, it’s not yet declared
        let _s = "hello"; // s is valid from this point forward
                          // do stuff with s
    } // this scope is now over, and s is no longer valid

    // Ways Variables and Data Interact: Move
    // If you’ve heard the terms shallow copy and deep copy while working with
    // other languages, the concept of copying the pointer, length, and capacity
    // without copying the data probably sounds like making a shallow copy.
    // But because Rust also invalidates the first variable, instead of calling
    // it a shallow copy, it’s known as a move.
    // In this example, we would say that s1 was moved into s2.
    // With only s2 valid, when it goes out of scope,
    // it alone will free the memory, and we’re done.
    let s1 = String::from("hello");
    let _s2 = s1;

    // To ensure memory safety, after the line let s2 = s1,
    // Rust considers s1 as no longer valid.
    // Therefore, Rust doesn’t need to free anything when s1 goes out of scope.
    //println!("{}, world!", s1);

    // Ways Variables and Data Interact: Clone
    // If we do want to deeply copy the heap data of the String,
    // not just the stack data, we can use a common method called clone.
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // Stack-Only Data: Copy
    // Integers that have a known size at compile time are stored entirely
    // on the stack, so copies of the actual values are quick to make.
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // Ownership and Functions
    {
        let s = String::from("hello"); // s comes into scope
        takes_ownership(s); // s's value moves into the function...
                            // ... and so is no longer valid here
        let x = 5; // x comes into scope
        makes_copy(x); // x would move into the function,
                       // but i32 is Copy, so it's okay to still
                       // use x afterward
    } // Here, x goes out of scope, then s. But because s's value was moved,
      // nothing special happens.

    // Return Values and Scope
    // Returning values can also transfer ownership.
    // The ownership of a variable follows the same pattern every time:
    // assigning a value to another variable moves it.
    // When a variable that includes data on the heap goes out of scope,
    // the value will be cleaned up by drop unless ownership of the data
    // has been moved to another variable.
    {
        let _s1 = gives_ownership(); // gives_ownership moves its return
                                     // value into s1
        let s2 = String::from("hello"); // s2 comes into scope
        let _s3 = takes_and_gives_back(s2); // s2 is moved into
                                            // takes_and_gives_back, which also
                                            // moves its return value into s3
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
      // happens. s1 goes out of scope and is dropped.

    // While this works, taking ownership and then returning ownership
    // with every function is a bit tedious.
    // It’s quite annoying that anything we pass in also needs to be passed
    // back if we want to use it again, in addition to any data resulting from
    // the body of the function that we might want to return as well.
    {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1);
        println!("The length of '{}' is {}.", s2, len);
    }
    // Luckily for us, Rust has a feature for using a value without
    // transferring ownership, called references.
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}
