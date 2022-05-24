use std::fmt::Display;

fn main() {
    // Traits: Defining Shared Behavior
    // A trait defines functionality a particular type has and can share
    // with other types. We can use traits to define shared behavior in
    // an abstract way. We can use trait bounds to specify that a generic
    // type can be any type that has certain behavior.
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());
    println!("1 new tweet: {}", tweet.summarize2());
    println!("1 new tweet: {}", tweet.summarize3());
    notify(&tweet);

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article.summarize());
    println!("New article available! {}", article.summarize2());
    println!("New article available! {}", article.summarize3());
    notify(&article);

    notify(&returns_summarizable());

    let pair = Pair::new(5, 10);
    pair.cmp_display();

    3.to_string();
}

// Defining a Trait
// A type’s behavior consists of the methods we can call on that type.
// Different types share the same behavior if we can call the same methods
// on all of those types. Trait definitions are a way to group method
// signatures together to define a set of behaviors necessary to
// accomplish some purpose.
// We want to make a media aggregator library crate named aggregator that
// can display summaries of data that might be stored in a NewsArticle
// or Tweet instance. To do this, we need a summary from each type,
// and we’ll request that summary by calling a summarize method on an instance.

// Here, we declare a trait using the trait keyword and then the trait’s name,
// which is Summary in this case. We’ve also declared the trait as pub so that
// crates depending on this crate can make use of this trait too, as we’ll see
// in a few examples. Inside the curly brackets, we declare the method signatures
// that describe the behaviors of the types that implement this trait, which in
// this case is fn summarize(&self) -> String.
pub trait Summary {
    // A trait can have multiple methods in its body:
    // the method signatures are listed one per line and each line ends in a semicolon.
    fn summarize(&self) -> String;

    // Default Implementations
    // Sometimes it’s useful to have default behavior for some or all of the
    // methods in a trait instead of requiring implementations for all methods
    // on every type. Then, as we implement the trait on a particular type,
    // we can keep or override each method’s default behavior.
    fn summarize2(&self) -> String {
        String::from("(Read more...)")
    }

    // Default implementations can call other methods in the same trait,
    // even if those other methods don’t have a default implementation.
    // In this way, a trait can provide a lot of useful functionality
    // and only require implementors to specify a small part of it.
    fn summarize_author(&self) -> String;
    fn summarize3(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// Implementing a Trait on a Type
// Now that we’ve defined the desired signatures of the Summary trait’s methods,
// we can implement it on the types in our media aggregator.
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Implementing a trait on a type is similar to implementing regular methods.
// The difference is that after impl, we put the trait name we want to implement,
// then use the for keyword, and then specify the name of the type we want
// to implement the trait for.
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Traits as Parameters
// Now that you know how to define and implement traits, we can explore
// how to use traits to define functions that accept many different types.
// We'll use the Summary trait we implemented on the NewsArticle and Tweet types
// to define a notify function that calls the summarize method on its item
// parameter, which is of some type that implements the Summary trait.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait Bound Syntax
// The impl Trait syntax works for straightforward cases but is actually
// syntax sugar for a longer form known as a trait bound.
// The impl Trait syntax is convenient and makes for more concise code
// in simple cases, while the fuller trait bound syntax can express more
// complexity in other cases.
//pub fn notify<T: Summary>(item: &T) {

// Using impl Trait is appropriate if we want this function to allow
// item1 and item2 to have different types.
//pub fn notify(item1: &impl Summary, item2: &impl Summary) {

// The generic type T specified as the type of the item1 and item2
// parameters constrains the function such that the concrete type of
// the value passed as an argument for item1 and item2 must be the same.
//pub fn notify<T: Summary>(item1: &T, item2: &T) {

// Specifying Multiple Trait Bounds with the + Syntax
// We can also specify more than one trait bound. Say we wanted notify
// to use display formatting as well as summarize on item: we specify
// in the notify definition that item must implement both Display and Summary.
// We can do so using the + syntax.
//pub fn notify(item: &(impl Summary + Display)) {

// The + syntax is also valid with trait bounds on generic types.
//pub fn notify<T: Summary + Display>(item: &T) {

// Clearer Trait Bounds with where Clauses
// Using too many trait bounds has its downsides. Each generic has its own
// trait bounds, so functions with multiple generic type parameters can contain
// lots of trait bound information between the function’s name and its parameter
// list, making the function signature hard to read. For this reason, Rust has
// alternate syntax for specifying trait bounds inside a where clause after
// the function signature.
//fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
//fn some_function<T, U>(t: &T, u: &U) -> i32
//  where T: Display + Clone, U: Clone + Debug {

// Returning Types that Implement Traits
// We can also use the impl Trait syntax in the return position
// to return a value of some type that implements a trait.
// However, you can only use impl Trait if you’re returning a single type.
// For example, this code that returns either a NewsArticle or a Tweet with
// the return type specified as impl Summary wouldn’t work.
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// Using Trait Bounds to Conditionally Implement Methods
// By using a trait bound with an impl block that uses generic type parameters,
// we can implement methods conditionally for types that implement the specified traits.
struct Pair<T> {
    x: T,
    y: T,
}

// Always implements the new function to return a new instance of Pair<T>.
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// But in the next impl block, Pair<T> only implements the cmp_display method
// if its inner type T implements the PartialOrd trait that enables comparison
// and the Display trait that enables printing.
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
