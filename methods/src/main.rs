#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Defining Methods
// Let’s change the area function that has a Rectangle instance as a parameter
// and instead make an area method defined on the Rectangle struct.
// To define the function within the context of Rectangle,
// we start an impl (implementation) block for Rectangle.
// Everything within this impl block will be associated with the Rectangle type.
// In the signature for area, we use &self instead of rectangle: &Rectangle.
// The &self is actually short for self: &Self. Within an impl block,
// the type Self is an alias for the type that the impl block is for.
impl Rectangle {
    // Methods must have a parameter named self of type Self
    // for their first parameter, so Rust lets you abbreviate this with
    // only the name self in the first parameter spot. Note that we still
    // need to use the & in front of the self shorthand to indicate this
    // method borrows the Self instance, just as we did in rectangle: &Rectangle.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Note that we can choose to give a method the same name
    // as one of the struct’s fields.
    // For example, we can define a method on Rectangle also named width.
    fn width(&self) -> bool {
        self.width > 0
    }

    // We know we want to define a method, so it will be within
    // the impl Rectangle block.
    // The method name will be can_hold, and it will take an immutable
    // borrow of another Rectangle as a parameter.
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated Functions
    // All functions defined within an impl block are called associated
    // functions because they’re associated with the type named after the impl.
    // We can define associated functions that don’t have self as their first
    // parameter (and thus are not methods) because they don’t need
    // an instance of the type to work with.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Multiple impl Blocks
// Each struct is allowed to have multiple impl blocks.
impl Rectangle {
    fn height(&self) -> bool {
        self.height > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // The method syntax goes after an instance:
    // we add a dot followed by the method name, parentheses, and any arguments.
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // To call this associated function, we use the :: syntax with the struct name.
    let _sq = Rectangle::square(3);

    if rect1.height() {
        println!("The rectangle has a nonzero height; it is {}", rect1.height);
    }
}
