// Using Trait Objects That Allow for Values of Different Types
// We mentioned that one limitation of vectors is that they can store elements
// of only one type. We created a workaround where we defined a SpreadsheetCell
// enum that had variants to hold integers, floats, and text.  This meant we
// could store different types of data in each cell and still have a vector that
// represented a row of cells. This is a perfectly good solution when our
// interchangeable items are a fixed set of types that we know when our code is
// compiled.

// However, sometimes we want our library user to be able to extend the set of
// types that are valid in a particular situation. To show how we might achieve
// this, we’ll create an example graphical user interface (GUI) tool that
// iterates through a list of items, calling a draw method on each one to draw
// it to the screen—a common technique for GUI tools. We’ll create a library
// crate called gui that contains the structure of a GUI library. This crate
// might include some types for people to use, such as Button or TextField. In
// addition, gui users will want to create their own types that can be drawn:
// for instance, one programmer might add an Image and another might add a
// SelectBox.

// We won’t implement a fully fledged GUI library for this example but will show
// how the pieces would fit together. At the time of writing the library, we
// can’t know and define all the types other programmers might want to create.
// But we do know that gui needs to keep track of many values of different
// types, and it needs to call a draw method on each of these differently typed
// values. It doesn’t need to know exactly what will happen when we call the
// draw method, just that the value will have that method available for us to
// call.

// To do this in a language with inheritance, we might define a class named
// Component that has a method named draw on it. The other classes, such as
// Button, Image, and SelectBox, would inherit from Component and thus inherit
// the draw method. They could each override the draw method to define their
// custom behavior, but the framework could treat all of the types as if they
// were Component instances and call draw on them. But because Rust doesn’t have
// inheritance, we need another way to structure the gui library to allow users
// to extend it with new types.

// Defining a Trait for Common Behavior
// To implement the behavior we want gui to have, we’ll define a trait named
// Draw that will have one method named draw. Then we can define a vector that
// takes a trait object. A trait object points to both an instance of a type
// implementing our specified trait as well as a table used to look up trait
// methods on that type at runtime. We create a trait object by specifying some
// sort of pointer, such as a & reference or a Box<T> smart pointer, then the
// dyn keyword, and then specifying the relevant trait.  We can use trait
// objects in place of a generic or concrete type. Wherever we use a trait
// object, Rust’s type system will ensure at compile time that any value used in
// that context will implement the trait object’s trait. Consequently, we don’t
// need to know all the possible types at compile time.

// We’ve mentioned that in Rust, we refrain from calling structs and enums
// “objects” to distinguish them from other languages’ objects. In a struct or
// enum, the data in the struct fields and the behavior in impl blocks are
// separated, whereas in other languages, the data and behavior combined into
// one concept is often labeled an object. However, trait objects are more like
// objects in other languages in the sense that they combine data and behavior.
// But trait objects differ from traditional objects in that we can’t add data
// to a trait object. Trait objects aren’t as generally useful as objects in
// other languages: their specific purpose is to allow abstraction across common
// behavior.
pub trait Draw {
    fn draw(&self);
}

// This vector is of type Box<dyn Draw>, which is a trait object; it’s a
// stand-in for any type inside a Box that implements the Draw trait.
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// On the Screen struct, we’ll define a method named run that will call the draw
// method on each of its components.
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// This works differently from defining a struct that uses a generic type
// parameter with trait bounds. A generic type parameter can only be substituted
// with one concrete type at a time, whereas trait objects allow for multiple
// concrete types to fill in for the trait object at runtime.
pub struct Screen2<T: Draw> {
    pub components: Vec<T>,
}

// This restricts us to a Screen instance that has a list of components all of
// type Button or all of type TextField. If you’ll only ever have homogeneous
// collections, using generics and trait bounds is preferable because the
// definitions will be monomorphized at compile time to use the concrete types.

// On the other hand, with the method using trait objects, one Screen instance
// can hold a Vec<T> that contains a Box<Button> as well as a Box<TextField>.
impl<T> Screen2<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Implementing the Trait
// Now we’ll add some types that implement the Draw trait. We’ll provide the
// Button type. Again, actually implementing a GUI library is beyond the scope
// of this book, so the draw method won’t have any useful implementation in its
// body.
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

// The width, height, and label fields on Button will differ from the fields on
// other components, such as a TextField type, that might have those fields plus
// a placeholder field instead. Each of the types we want to draw on the screen
// will implement the Draw trait but will use different code in the draw method
// to define how to draw that particular type, as Button has here (without the
// actual GUI code, which is beyond the scope of this chapter). The Button type,
// for instance, might have an additional impl block containing methods related
// to what happens when a user clicks the button. These kinds of methods won’t
// apply to types like TextField.
impl Draw for Button {
    fn draw(&self) {
        println!(
            "Button w:{}, h:{}, label:{}",
            self.width, self.height, self.label
        );
    }
}
