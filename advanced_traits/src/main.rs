#![allow(dead_code)]
use std::fmt;
use std::ops::Add;

// Specifying Placeholder Types in Trait Definitions with Associated Types
// Associated types connect a type placeholder with a trait such that the trait
// method definitions can use these placeholder types in their signatures. The
// implementor of a trait will specify the concrete type to be used in this
// type’s place for the particular implementation. That way, we can define a
// trait that uses some types without needing to know exactly what those types
// are until the trait is implemented.

// We’ve described most of the advanced features in this chapter as being rarely
// needed. Associated types are somewhere in the middle: they’re used more
// rarely than features explained in the rest of the book but more commonly than
// many of the other features discussed in this chapter.

// One example of a trait with an associated type is the Iterator trait that the
// standard library provides. The associated type is named Item and stands in
// for the type of the values the type implementing the Iterator trait is
// iterating over.

// The type Item is a placeholder type, and the next method’s definition shows
// that it will return values of type Option<Self::Item>. Implementors of the
// Iterator trait will specify the concrete type for Item, and the next method
// will return an Option containing a value of that concrete type.
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

// The difference is that when using generics, we must annotate the types in
// each implementation; because we can also implement Iterator<String> for
// Counter or any other type, we could have multiple implementations of Iterator
// for Counter. In other words, when a trait has a generic parameter, it can be
// implemented for a type multiple times, changing the concrete types of the
// generic type parameters each time. When we use the next method on Counter, we
// would have to provide type annotations to indicate which implementation of
// Iterator we want to use.
pub trait IteratorGeneric<T> {
    fn next(&mut self) -> Option<T>;
}

// With associated types, we don’t need to annotate types because we can’t
// implement a trait on a type multiple times. With the definition that uses
// associated types, we can only choose what the type of Item will be once,
// because there can only be one impl Iterator for Counter.  We don’t have to
// specify that we want an iterator of u32 values everywhere that we call next
// on Counter.
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// Default Generic Type Parameters and Operator Overloading
// When we use generic type parameters, we can specify a default concrete type
// for the generic type. This eliminates the need for implementors of the trait
// to specify a concrete type if the default type works. The syntax for
// specifying a default type for a generic type is
// <PlaceholderType=ConcreteType> when declaring the generic type.

// A great example of a situation where this technique is useful is with
// operator overloading. Operator overloading is customizing the behavior of an
// operator (such as +) in particular situations.

// Rust doesn’t allow you to create your own operators or overload arbitrary
// operators. But you can overload the operations and corresponding traits
// listed in std::ops by implementing the traits associated with the operator.
// For example, in Listing 19-14 we overload the + operator to add two Point
// instances together. We do this by implementing the Add trait on a Point
// struct:
#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// The default generic type in this code is within the Add trait. Here is its
// definition:
#[allow(unused)]
fn definition() {
    // This code should look generally familiar: a trait with one method and an
    // associated type. The new part is Rhs=Self: this syntax is called default
    // type parameters. The Rhs generic type parameter (short for “right hand
    // side”) defines the type of the rhs parameter in the add method. If we
    // don’t specify a concrete type for Rhs when we implement the Add trait,
    // the type of Rhs will default to Self, which will be the type we’re
    // implementing Add on.
    trait Add<Rhs = Self> {
        type Output;
        fn add(self, rhs: Rhs) -> Self::Output;
    }
}

// When we implemented Add for Point, we used the default for Rhs because we
// wanted to add two Point instances. Let’s look at an example of implementing
// the Add trait where we want to customize the Rhs type rather than using the
// default.
impl Add for Point {
    type Output = Point;
    // The add method adds the x values of two Point instances and the y values
    // of two Point instances to create a new Point. The Add trait has an
    // associated type named Output that determines the type returned from the
    // add method.
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// We have two structs, Millimeters and Meters, holding values in different
// units. This thin wrapping of an existing type in another struct is known as
// the newtype pattern. We want to add values in millimeters to values in meters
// and have the implementation of Add do the conversion correctly. We can
// implement Add for Millimeters with Meters as the Rhs.
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// To add Millimeters and Meters, we specify impl Add<Meters> to set the value
// of the Rhs type parameter instead of using the default of Self.

// You’ll use default type parameters in two main ways:
//  To extend a type without breaking existing code
//  To allow customization in specific cases most users won’t need

// The standard library’s Add trait is an example of the second purpose:
// usually, you’ll add two like types, but the Add trait provides the ability to
// customize beyond that. Using a default type parameter in the Add trait
// definition means you don’t have to specify the extra parameter most of the
// time. In other words, a bit of implementation boilerplate isn’t needed,
// making it easier to use the trait.

// The first purpose is similar to the second but in reverse: if you want to add
// a type parameter to an existing trait, you can give it a default to allow
// extension of the functionality of the trait without breaking the existing
// implementation code.

// Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
// Nothing in Rust prevents a trait from having a method with the same name as
// another trait’s method, nor does Rust prevent you from implementing both
// traits on one type. It’s also possible to implement a method directly on the
// type with the same name as methods from traits.

// When calling methods with the same name, you’ll need to tell Rust which one
// you want to use. Consider the code in Listing 19-16 where we’ve defined two
// traits, Pilot and Wizard, that both have a method called fly. We then
// implement both traits on a type Human that already has a method named fly
// implemented on it. Each fly method does something different.
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// Because the fly method takes a self parameter, if we had two types that both
// implement one trait, Rust could figure out which implementation of a trait to
// use based on the type of self.

// However, associated functions that are not methods don’t have a self
// parameter. When there are multiple types or traits that define non-method
// functions with the same function name, Rust doesn't always know which type
// you mean unless you use fully qualified syntax. For example, the Animal trait
// has the associated non-method function baby_name, and the Animal trait is
// implemented for the struct Dog. There’s also an associated non-method
// function baby_name defined on Dog directly.
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// Using Supertraits to Require One Trait’s Functionality Within Another Trait
// Sometimes, you might need one trait to use another trait’s functionality. In
// this case, you need to rely on the dependent trait also being implemented.
// The trait you rely on is a supertrait of the trait you’re implementing.

// For example, let’s say we want to make an OutlinePrint trait with an
// outline_print method that will print a value framed in asterisks.

// In the implementation of outline_print, we want to use the Display trait’s
// functionality. Therefore, we need to specify that the OutlinePrint trait will
// work only for types that also implement Display and provide the functionality
// that OutlinePrint needs. We can do that in the trait definition by specifying
// OutlinePrint: Display. This technique is similar to adding a trait bound to
// the trait.

// Because we’ve specified that OutlinePrint requires the Display trait, we can
// use the to_string function that is automatically implemented for any type
// that implements Display. If we tried to use to_string without adding a colon
// and specifying the Display trait after the trait name, we’d get an error
// saying that no method named to_string was found for the type &Self in the
// current scope.
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// Let’s see what happens when we try to implement OutlinePrint on a type that
// doesn’t implement Display.  We get an error saying that Display is required
// but not implemented.
impl OutlinePrint for Point {}

// To fix this, we implement Display on Point and satisfy the constraint that
// OutlinePrint requires, like so:
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Using the Newtype Pattern to Implement External Traits on External Types
// We mentioned the orphan rule that states we’re allowed to implement a trait
// on a type as long as either the trait or the type are local to our crate.
// It’s possible to get around this restriction using the newtype pattern, which
// involves creating a new type in a tuple struct.  The tuple struct will have
// one field and be a thin wrapper around the type we want to implement a trait
// for. Then the wrapper type is local to our crate, and we can implement the
// trait on the wrapper. Newtype is a term that originates from the Haskell
// programming language. There is no runtime performance penalty for using this
// pattern, and the wrapper type is elided at compile time.

// As an example, let’s say we want to implement Display on Vec<T>, which the
// orphan rule prevents us from doing directly because the Display trait and the
// Vec<T> type are defined outside our crate. We can make a Wrapper struct that
// holds an instance of Vec<T>; then we can implement Display on Wrapper and use
// the Vec<T> value.
struct Wrapper(Vec<String>);

// The implementation of Display uses self.0 to access the inner Vec<T>, because
// Wrapper is a tuple struct and Vec<T> is the item at index 0 in the tuple.
// Then we can use the functionality of the Display type on Wrapper.
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// The downside of using this technique is that Wrapper is a new type, so it
// doesn’t have the methods of the value it’s holding. We would have to
// implement all the methods of Vec<T> directly on Wrapper such that the methods
// delegate to self.0, which would allow us to treat Wrapper exactly like a
// Vec<T>.

// If we wanted the new type to have every method the inner type has,
// implementing the Deref trait on the Wrapper to return the inner type would be
// a solution. If we don’t want the Wrapper type to have all the methods of the
// inner type—for example, to restrict the Wrapper type’s behavior—we would have
// to implement just the methods we do want manually.

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // Running this code will print *waving arms furiously*, showing that Rust
    // called the fly method implemented on Human directly.
    let person = Human;
    person.fly();

    // To call the fly methods from either the Pilot trait or the Wizard trait,
    // we need to use more explicit syntax to specify which fly method we mean.
    let person = Human;
    // Specifying the trait name before the method name clarifies to Rust which
    // implementation of fly we want to call. We could also write
    // Human::fly(&person), which is equivalent to the person.fly(), but this is
    // a bit longer to write if we don’t need to disambiguate.
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    // This code is for an animal shelter that wants to name all puppies Spot,
    // which is implemented in the baby_name associated function that is defined
    // on Dog. The Dog type also implements the trait Animal, which describes
    // characteristics that all animals have. Baby dogs are called puppies, and
    // that is expressed in the implementation of the Animal trait on Dog in the
    // baby_name function associated with the Animal trait.
    println!("A baby dog is called a {}", Dog::baby_name());

    // This output isn’t what we wanted. We want to call the baby_name function
    // that is part of the Animal trait that we implemented on Dog so the code
    // prints A baby dog is called a puppy.

    // To disambiguate and tell Rust that we want to use the implementation of
    // Animal for Dog as opposed to the implementation of Animal for some other
    // type, we need to use fully qualified syntax.
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // Then implementing the OutlinePrint trait on Point will compile
    // successfully, and we can call outline_print on a Point instance to
    // display it within an outline of asterisks.
    let p = Point { x: 1, y: 3 };
    p.outline_print();

    // Now you know how the newtype pattern is used in relation to traits; it’s
    // also a useful pattern even when traits are not involved.
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
