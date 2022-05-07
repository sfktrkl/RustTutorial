fn main() {
    // Integer types
    let _x = 2; // i32
    let _y: i64 = 5; // i64

    // Floating point types
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // Numeric operations
    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // Results in 0

    // remainder
    let _remainder = 43 % 5;

    // Booleans
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // Character type
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    // Tuple type
    // A tuple is a general way of grouping together a number of values with
    // a variety of types into one compound type. Tuples have a fixed length:
    // once declared, they cannot grow or shrink in size.
    let _tup: (i32, f64, u8) = (500, 6.4, 1);

    // The variable tup binds to the entire tuple, because a tuple is considered
    // a single compound element. To get the individual values out of a tuple,
    // we can use pattern matching to destructure a tuple value.
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    // We can also access a tuple element directly by using a period (.)
    // followed by the index of the value we want to access.
    let _x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = _x.0;
    let _six_point_four = _x.1;
    let _one = _x.2;

    // The Array Type
    // Another way to have a collection of multiple values is with an array.
    // Unlike a tuple, every element of an array must have the same type.
    // Unlike arrays in some other languages, arrays in Rust have a fixed length.
    let _a = [1, 2, 3, 4, 5];

    // You write an array’s type using square brackets with the type of
    // each element, a semicolon, and then the number of elements in the array.
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // You can also initialize an array to contain the same value for
    // each element by specifying the initial value, followed by a semicolon,
    // and then the length of the array in square brackets.
    let _a = [3; 5];
    let _a = [3, 3, 3, 3, 3];

    // An array is a single chunk of memory of a known, fixed size that can
    // be allocated on the stack. You can access elements of an array using indexing.
    let _first = _a[0];
    let _second = _a[1];
}
