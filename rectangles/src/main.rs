// Refactoring with Structs: Adding More Meaning
// We use structs to add meaning by labeling the data.
// We can transform the tuple we’re using into a struct with a name
// for the whole as well as names for the parts.
// Rust does include functionality to print out debugging information,
// but we have to explicitly opt in to make that functionality
// available for our struct.
// In addition to the Debug trait, Rust has provided a number of traits
// for us to use with the derive attribute that can
// add useful behavior to our custom types.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // An Example Program Using Structs
    // To understand when we might want to use structs,
    // let’s write a program that calculates the area of a rectangle.
    // We’ll start by using single variables, and then refactor
    // the program until we’re using structs instead.
    {
        let width1 = 30;
        let height1 = 50;

        println!(
            "The area of the rectangle is {} square pixels.",
            area(width1, height1)
        );
    }

    // Refactoring with Tuples
    // In one way, this program is better. Tuples let us add a bit of structure,
    // and we’re now passing just one argument.
    // But in another way, this version is less clear:
    // tuples don’t name their elements, so we have to index into
    // the parts of the tuple, making our calculation less obvious.
    {
        let rect1 = (30, 50);

        println!(
            "The area of the rectangle is {} square pixels.",
            area_tuple(rect1)
        );
    }

    // Here we’ve defined a struct and named it Rectangle.
    // Inside the curly brackets, we defined the fields as width and height,
    // both of which have type u32.
    {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            area_struct(&rect1)
        );

        // Adding Useful Functionality with Derived Traits
        // It’d be useful to be able to print an instance of Rectangle while
        // we’re debugging our program and see the values for all its fields.
        // The println! macro can do many kinds of formatting, and by default,
        // the curly brackets tell println! to use formatting known as Display:
        // output intended for direct end user consumption.
        // The primitive types we’ve seen so far implement Display by default,
        // because there’s only one way you’d want to show a 1 or any
        // other primitive type to a user.
        // The println! macro call will now look like
        //println!("rect1 is {:?}", rect1);. Putting the specifier :?
        // inside the curly brackets tells println! we want to use
        // an output format called Debug.
        println!("rect1 is {:?}", rect1);
        // When we have larger structs, it’s useful to have output that’s
        // a bit easier to read; in those cases, we can use {:#?} instead of {:?}
        // in the println! string.
        println!("rect1 is {:#?}", rect1);

        // Another way to print out a value using the Debug format
        // is to use the dbg! macro, which takes ownership of an expression,
        // prints the file and line number of where that dbg! macro call occurs
        // in your code along with the resulting value of that expression,
        // and returns ownership of the value.
        dbg!(&rect1);
    }
}

// The area function is supposed to calculate the area of one rectangle,
// but the function we wrote has two parameters, and it’s not clear anywhere
// in our program that the parameters are related.
// It would be more readable and more manageable to group width and height together.
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// Our area function is now defined with one parameter,
// which we’ve named rectangle, whose type is an immutable
// borrow of a struct Rectangle instance.
fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
