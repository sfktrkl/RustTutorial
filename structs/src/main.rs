// Defining and Instantiating Structs
// Structs are similar to tuples, discussed in “The Tuple Type” section,
// in that both hold multiple related values. Like tuples, the pieces of
// a struct can be different types. Unlike with tuples, in a struct you’ll
// name each piece of data so it’s clear what the values mean.
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Using Tuple Structs without Named Fields to Create Different Types
// Rust also supports structs that look similar to tuples, called tuple structs.
// Tuple structs have the added meaning the struct name provides but don’t have
// names associated with their fields; rather, they just have the types of the fields.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like Structs Without Any Fields
// You can also define structs that don’t have any fields!
// These are called unit-like structs because they behave similarly to ().
// Unit-like structs can be useful when you need to implement a trait
// on some type but don’t have any data that you want to store in the type itself.
struct AlwaysEqual;

fn main() {
    // To use a struct after we’ve defined it, we create an instance of that
    // struct by specifying concrete values for each of the fields. We create
    // an instance by stating the name of the struct and then add curly brackets
    // containing key: value pairs, where the keys are the names of the fields
    // and the values are the data we want to store in those fields. We don’t
    // have to specify the fields in the same order in which we declared them
    // in the struct.
    // Note that the entire instance must be mutable;
    // Rust doesn’t allow us to mark only certain fields as mutable.
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // To get a specific value from a struct, we use dot notation.
    user1.email = String::from("anotheremail@example.com");
    user1.username = String::from("someusername1234");
    user1.active = false;
    user1.sign_in_count = 2;

    let _user2 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    let _user2 = build_user_shorthand(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    // Creating Instances From Other Instances With Struct Update Syntax
    // It’s often useful to create a new instance of a struct that includes
    // most of the values from another instance, but changes some.
    // You can do this using struct update syntax.
    // In this example, we can no longer use user1 after creating user2
    // because the String in the username field of user1 was moved into user2.
    let _user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // Note that the black and origin values are different types, because
    // they’re instances of different tuple structs. Each struct you define
    // is its own type, even though the fields within the struct have the same types.
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // To define AlwaysEqual, we use the struct keyword, the name we want,
    // then a semicolon. No need for curly brackets or parentheses!
    let _subject = AlwaysEqual;
}

// It makes sense to name the function parameters with the same name as
// the struct fields, but having to repeat the email and username field
// names and variables is a bit tedious. If the struct had more fields,
// repeating each name would get even more annoying.
// Luckily, there’s a convenient shorthand!
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// Because the parameter names and the struct field names are exactly
// the same, we can use the field init shorthand syntax.
fn build_user_shorthand(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
