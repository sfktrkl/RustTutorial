fn main() {
    // The issue with the tuple code in Listing 4-5 is that we have to return
    // the String to the calling function so we can still use the String after
    // the call to calculate_length, because the String was moved into calculate_length.
    // Instead, we can provide a reference to the String value. A reference
    // is like a pointer in that it’s an address we can follow to access data
    // stored at that address that is owned by some other variable.
    // Unlike a pointer, a reference is guaranteed to point to a valid value
    // of a particular type.
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    // The &s1 syntax lets us create a reference that refers to the value of
    // s1 but does not own it. Because it does not own it, the value it points
    // to will not be dropped when the reference stops being used.
    println!("The length of '{}' is {}.", s1, len);

    // Just as variables are immutable by default, so are references.
    // We’re not allowed to modify something we have a reference to.
    //let s = String::from("hello");
    //change(&s);

    // Mutable References
    // First, we change s to be mut. Then we create a mutable reference with
    // &mut s where we call the change function, and update the function
    // signature to accept a mutable reference with some_string: &mut String.
    // This makes it very clear that the change function will mutate
    // the value it borrows.
    let mut s = String::from("hello");
    change(&mut s);

    // Mutable references have one big restriction:
    // you can have only one mutable reference to a particular piece of data at a time.
    // The benefit of having this restriction is that Rust can prevent
    // data races at compile time. A data race is similar to a race
    // condition and happens when these three behaviors occur:
    //  Two or more pointers access the same data at the same time.
    //  At least one of the pointers is being used to write to the data.
    //  There’s no mechanism being used to synchronize access to the data.
    let mut s = String::from("hello");
    let _r1 = &mut s;
    let _r2 = &mut s;
    //println!("{}, {}", _r1, _r2);

    // As always, we can use curly brackets to create a new scope,
    // allowing for multiple mutable references
    let mut s = String::from("hello");
    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.
    let _r2 = &mut s;

    // Rust enforces a similar rule for combining mutable and immutable references.
    // We also cannot have a mutable reference while we have an immutable
    // one to the same value. Users of an immutable reference don’t expect
    // the value to suddenly change out from under them!
    // However, multiple immutable references are allowed because no one
    // who is just reading the data has the ability to affect anyone
    // else’s reading of the data.
    let mut s = String::from("hello");
    let _r1 = &s; // no problem
    let _r2 = &s; // no problem
    let _r3 = &mut s; // BIG PROBLEM
                      //println!("{}, {}, and {}", _r1, _r2, _r3);

    // Note that a reference’s scope starts from where it is introduced
    // and continues through the last time that reference is used.
    // For instance, this code will compile because the last usage of the
    // immutable references, the println!, occurs before the mutable reference
    // is introduced.
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    let r3 = &mut s; // no problem
    println!("{}", r3);

    // Dangling References
    // In languages with pointers, it’s easy to erroneously create
    // a dangling pointer--a pointer that references a location in memory
    // that may have been given to someone else--by freeing some memory
    // while preserving a pointer to that memory. In Rust, by contrast,
    // the compiler guarantees that references will never be dangling references.
    let _reference_to_nothing = dangle();
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> String {
    //&String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String
    s
    //&s // we return a reference to the String, s
    // The solution here is to return the String directly
    // This works without any problems. Ownership is moved out, and nothing is deallocated.
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
