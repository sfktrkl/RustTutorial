#![allow(dead_code)]

fn main() {
    // Generic Data Types
    // We use generics to create definitions for items like function signatures
    // or structs, which we can then use with many different concrete data types.
    // Let’s first look at how to define functions, structs, enums, and methods
    // using generics. Then we’ll discuss how generics affect code performance.
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer = Point { x: 5, y: 10 };
    println!("x: {}, y: {}", integer.x, integer.y);
    let float = Point { x: 1.0, y: 4.0 };
    println!("x: {}, y: {}", float.x, float.y);

    // The fields x and y must be the same type because
    // both have the same generic data type T.
    //let wont_work = Point { x: 5, y: 4.0 };

    let both_integer = Point2 { x: 5, y: 10 };
    println!("x: {}, y: {}", both_integer.x, both_integer.y);
    let both_float = Point2 { x: 1.0, y: 4.0 };
    println!("x: {}, y: {}", both_float.x, both_float.y);
    let integer_and_float = Point2 { x: 5, y: 4.0 };
    println!("x: {}, y: {}", integer_and_float.x, integer_and_float.y);

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p = Point { x: 5.0, y: 10.0 };
    println!("Distance from origin = {}", p.distance_from_origin());

    let p1 = Point3 { x: 5, y: 10.4 };
    let p2 = Point3 { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // Performance of Code Using Generics
    // You might be wondering whether there is a runtime cost when using generic
    // type parameters. The good news is that using generic types won't make
    // your run any slower than it would with concrete types.
    // Rust accomplishes this by performing monomorphization of the code
    // using generics at compile time. Monomorphization is the process of
    // turning generic code into specific code by filling in the concrete
    // types that are used when compiled.
}

// In Function Definitions
// When defining a function that uses generics, we place the generics
// in the signature of the function where we would usually specify
// the data types of the parameters and return value. Doing so makes
// our code more flexible and provides more functionality to callers
// of our function while preventing code duplication.
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = &item;
        }
    }
    largest
}

// In Struct Definitions
// We can also define structs to use a generic type parameter
// in one or more fields using the <> syntax.
struct Point<T> {
    x: T,
    y: T,
}

// To define a Point struct where x and y are both generics
// but could have different types, we can use multiple generic type parameters.
struct Point2<T, U> {
    x: T,
    y: U,
}

// In Enum Definitions
// As we did with structs, we can define enums to hold generic
// data types in their variants. Let’s take another look at the Option<T>
// enum that the standard library provides.

// This definition should now make more sense to you. As you can see,
// the Option<T> enum is generic over type T and has two variants:
// Some, which holds one value of type T and
// None variant that doesn’t hold any value.
enum Option<T> {
    Some(T),
    None,
}

// The Result enum is generic over two types, T and E, and has two variants:
// Ok, which holds a value of type T and
// Err, which holds a value of type E.
// This definition makes it convenient to use the Result
// enum anywhere we have an operation that might succeed.
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// In Method Definitions
// We can implement methods on structs and enums and use generic types
// in their definitions, too.
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// We can also specify constraints on generic types when defining
// methods on the type. We could, for example, implement methods only on
// Point<f32> instances rather than on Point<T> instances with any generic type.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Generic type parameters in a struct definition aren’t always the same as
// those you use in that same struct’s method signatures. It uses the generic
// types X1 and Y1 for the Point struct and X2 Y2 for the mixup method
// signature to make the example clearer.
struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point3<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}
