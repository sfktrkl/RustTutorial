fn main() {
    // Creating a New Vector
    // Note that we added a type annotation here. Because we aren’t inserting
    // any values into this vector, Rust doesn’t know what kind of elements
    // we intend to store. This is an important point.
    // Vectors are implemented using generics.
    let _v: Vec<i32> = Vec::new();

    // More often, you’ll create a Vec<T> with initial values and Rust will
    // infer the type of value you want to store, so you rarely need to do
    // this type annotation. Rust conveniently provides the vec! macro,
    // which will create a new vector that holds the values you give it.
    let _v = vec![1, 2, 3];

    // Updating a Vector
    let mut v = Vec::new();
    v.push(5);

    // Dropping a Vector Drops Its Elements
    // Like any other struct, a vector is freed when it goes out of scope.
    {
        let _v = vec![1, 2, 3, 4];
    } // v goes out of scope and is freed here

    // Reading Elements of Vectors
    // There are two ways to reference a value stored in a vector:
    // via indexing or using the get method.
    let v = vec![1, 2, 3, 4, 5];

    // We use the index value of 2 to get the third element
    // because vectors are indexed by number, starting at zero.
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // we get the third element by either using & and [],
    // which gives us a reference, or using the get method with
    // the index passed as an argument, which gives us an Option<&T>.
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // When we run this code, the first [] method will cause the program
    // to panic because it references a nonexistent element.
    // This method is best used when you want your program to crash
    // if there’s an attempt to access an element past the end of the vector.
    let _does_not_exist = &v[100];

    // When the get method is passed an index that is outside the vector,
    // it returns None without panicking. You would use this method if
    // accessing an element beyond the range of the vector may
    // happen occasionally under normal circumstances.
    let _does_not_exist = v.get(100);

    // When the program has a valid reference, the borrow checker
    // enforces the ownership and borrowing rules to ensure this
    // reference and any other references to the contents of the vector remain valid.
    // This error is due to the way vectors work: because vectors put the
    // values next to each other in memory, adding a new element onto
    // the end of the vector might require allocating new memory and
    // copying the old elements to the new space.
    //let _first = &v[0]; // immutable borrow occurs here
    //v.push(6);  // mutable borrow occurs here
    //println!("The first element is: {}", _first);

    // Iterating over the Values in a Vector
    // To access each element in a vector in turn, we would iterate through
    // all of the elements rather than use indices to access one at a time.
    for i in &v {
        println!("{}", i);
    }

    // We can also iterate over mutable references to each element in
    // a mutable vector in order to make changes to all the elements.
    // To change the value that the mutable reference refers to,
    // we have to use the * dereference operator to get to the value
    // in i before we can use the += operator.
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // Using an Enum to Store Multiple Types
    // Vectors can only store values that are the same type.
    // This can be inconvenient; there are definitely use cases for needing
    // to store a list of items of different types. Fortunately, the variants
    // of an enum are defined under the same enum type, so when we need one type
    // to represent elements of different types, we can define and use an enum!
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // Rust needs to know what types will be in the vector at compile time
    // so it knows exactly how much memory on the heap will be needed to
    // store each element. We must also be explicit about what types are
    // allowed in this vector. If Rust allowed a vector to hold any type,
    // there would be a chance that one or more of the types would cause
    // errors with the operations performed on the elements of the vector.
    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
