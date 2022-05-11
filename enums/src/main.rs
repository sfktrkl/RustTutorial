#![allow(dead_code)]
// Defining an Enum
// Enums are a way of defining custom data types in a different way than you do with structs.
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// Using enums has even more advantages. Thinking more about our IP address type,
// at the moment we don’t have a way to store the actual IP address data;
// we only know what kind it is.
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// However, representing the same concept using just an enum is more concise:
// rather than an enum inside a struct, we can put data directly into each enum
// variant. This new definition of the IpAddr enum says that both V4 and V6
// variants will have associated String values
enum IpAddr2 {
    V4(String),
    V6(String),
}

// There’s another advantage to using an enum rather than a struct:
// each variant can have different types and amounts of associated data.
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// This code illustrates that you can put any kind of data inside an enum variant:
// strings, numeric types, or structs, for example. You can even include another enum!
struct Ipv4Addr {}
struct Ipv6Addr {}

enum IpAddr4 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

// This enum has four variants with different types:
//  * Quit has no data associated with it at all.
//  * Move has named fields like a struct does.
//  * Write includes a single String.
//  * ChangeColor includes three i32 values.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// The following structs could hold the same data that the preceding enum variants hold
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// There is one more similarity between enums and structs: just as we’re able
// to define methods on structs using impl, we’re also able to define methods on enums.
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {
    // Enum Values
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("{:?}", home.kind);
    println!("{}", home.address);

    let _home = IpAddr2::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr2::V6(String::from("::1"));

    let _home = IpAddr3::V4(127, 0, 0, 1);
    let _loopback = IpAddr3::V6(String::from("::1"));

    let w = Message::Write(String::from("hello"));
    w.call();
    let _q = Message::Quit;
    let _m = Message::Move { x: 1, y: 2 };
    let _c = Message::ChangeColor(1, 2, 3);

    // As such, Rust does not have nulls, but it does have an enum that can encode
    // the concept of a value being present or absent. This enum is Option<T>,
    // and it is defined by the standard library as follows
    // <T> means the Some variant of the Option enum can hold one piece of data
    // of any type, and that each concrete type that gets used in place of T
    // makes the overall Option<T> type a different type. Here are some examples
    // of using Option values to hold number types and string types.
    let _some_number = Some(5);
    let _some_string = Some("a string");
    let _absent_number: Option<i32> = None;

    // When we have a Some value, we know that a value is present and the value
    // is held within the Some. When we have a None value, in some sense,
    // it means the same thing as null: we don’t have a valid value.
    // So why is having Option<T> any better than having null?

    // In short, because Option<T> and T (where T can be any type) are different
    // types, the compiler won’t let us use an Option<T> value as if it were
    // definitely a valid value. For example, this code won’t compile because
    // it’s trying to add an i8 to an Option<i8>:
    let _x: i8 = 5;
    let _y: Option<i8> = Some(5);
    //let sum = x + y;
}

// Note that the variants of the enum are namespaced under its identifier,
// and we use a double colon to separate the two. This is useful because now
// both values IpAddrKind::V4 and IpAddrKind::V6 are of the same type:
// IpAddrKind. We can then, for instance, define a function that takes any IpAddrKind
fn route(_ip_kind: IpAddrKind) {}
