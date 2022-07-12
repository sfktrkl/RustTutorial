use std::slice;

// Unsafe Rust
// All the code we’ve discussed so far has had Rust’s memory safety guarantees
// enforced at compile time. However, Rust has a second language hidden inside
// it that doesn’t enforce these memory safety guarantees: it’s called unsafe
// Rust and works just like regular Rust, but gives us extra superpowers.

// Unsafe Rust exists because, by nature, static analysis is conservative. When
// the compiler tries to determine whether or not code upholds the guarantees,
// it’s better for it to reject some valid programs rather than accept some
// invalid programs. Although the code might be okay, if the Rust compiler
// doesn’t have enough information to be confident, it will reject the code. In
// these cases, you can use unsafe code to tell the compiler, “Trust me, I know
// what I’m doing.” The downside is that you use it at your own risk: if you use
// unsafe code incorrectly, problems due to memory unsafety, such as null
// pointer dereferencing, can occur.

// Another reason Rust has an unsafe alter ego is that the underlying computer
// hardware is inherently unsafe. If Rust didn’t let you do unsafe operations,
// you couldn’t do certain tasks. Rust needs to allow you to do low-level
// systems programming, such as directly interacting with the operating system
// or even writing your own operating system. Working with low-level systems
// programming is one of the goals of the language. Let’s explore what we can do
// with unsafe Rust and how to do it.

// Unsafe Superpowers
// To switch to unsafe Rust, use the unsafe keyword and then start a new block
// that holds the unsafe code. You can take five actions in unsafe Rust, called
// unsafe superpowers, that you can’t in safe Rust. Those superpowers include
// the ability to:
//  Dereference a raw pointer
//  Call an unsafe function or method
//  Access or modify a mutable static variable
//  Implement an unsafe trait
//  Access fields of unions

// It’s important to understand that unsafe doesn’t turn off the borrow checker
// or disable any other of Rust’s safety checks: if you use a reference in
// unsafe code, it will still be checked. The unsafe keyword only gives you
// access to these five features that are then not checked by the compiler for
// memory safety. You’ll still get some degree of safety inside of an unsafe
// block.

// In addition, unsafe does not mean the code inside the block is necessarily
// dangerous or that it will definitely have memory safety problems: the intent
// is that as the programmer, you’ll ensure the code inside an unsafe block will
// access memory in a valid way.

// People are fallible, and mistakes will happen, but by requiring these five
// unsafe operations to be inside blocks annotated with unsafe you’ll know that
// any errors related to memory safety must be within an unsafe block. Keep
// unsafe blocks small; you’ll be thankful later when you investigate memory
// bugs.

// To isolate unsafe code as much as possible, it’s best to enclose unsafe code
// within a safe abstraction and provide a safe API, which we’ll discuss later
// in the chapter when we examine unsafe functions and methods. Parts of the
// standard library are implemented as safe abstractions over unsafe code that
// has been audited. Wrapping unsafe code in a safe abstraction prevents uses of
// unsafe from leaking out into all the places that you or your users might want
// to use the functionality implemented with unsafe code, because using a safe
// abstraction is safe.

fn main() {
    // Dereferencing a Raw Pointer
    // Unsafe Rust has two new types called raw pointers that are similar to
    // references. As with references, raw pointers can be immutable or mutable
    // and are written as *const T and *mut T, respectively. The asterisk isn’t
    // the dereference operator; it’s part of the type name. In the context of
    // raw pointers, immutable means that the pointer can’t be directly assigned
    // to after being dereferenced.

    // Different from references and smart pointers, raw pointers:
    //  Are allowed to ignore the borrowing rules by having both immutable and
    //      mutable pointers or multiple mutable pointers to the same location
    //  Aren’t guaranteed to point to valid memory
    //  Are allowed to be null
    //  Don’t implement any automatic cleanup

    // By opting out of having Rust enforce these guarantees, you can give up
    // guaranteed safety in exchange for greater performance or the ability to
    // interface with another language or hardware where Rust’s guarantees don’t
    // apply.
    let mut num = 5;
    let _r1 = &num as *const i32;
    // We’ve created raw pointers by using as to cast an immutable and a mutable
    // reference into their corresponding raw pointer types. Because we created
    // them directly from references guaranteed to be valid, we know these
    // particular raw pointers are valid, but we can’t make that assumption
    // about just any raw pointer.
    let _r2 = &mut num as *mut i32;

    // Next, we’ll create a raw pointer whose validity we can’t be so certain
    // of. To create a raw pointer to an arbitrary location in memory. Trying to
    // use arbitrary memory is undefined: there might be data at that address or
    // there might not, the compiler might optimize the code so there is no
    // memory access, or the program might error with a segmentation fault.
    // Usually, there is no good reason to write code like this, but it is
    // possible.
    let address = 0x012345usize;
    let _r = address as *const i32;

    // Recall that we can create raw pointers in safe code, but we can’t
    // dereference raw pointers and read the data being pointed to.
    // Creating a pointer does no harm; it’s only when we try to access the
    // value that it points at that we might end up dealing with an invalid
    // value.
    let mut num = 5;
    // We created *const i32 and *mut i32 raw pointers that both pointed to the
    // same memory location, where num is stored. If we instead tried to create
    // an immutable and a mutable reference to num, the code would not have
    // compiled because Rust’s ownership rules don’t allow a mutable reference
    // at the same time as any immutable references. With raw pointers, we can
    // create a mutable pointer and an immutable pointer to the same location
    // and change data through the mutable pointer, potentially creating a data
    // race. Be careful!
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // Calling an Unsafe Function or Method
    // The second type of operation that requires an unsafe block is calls to unsafe
    // functions. Unsafe functions and methods look exactly like regular functions
    // and methods, but they have an extra unsafe before the rest of the definition.
    // The unsafe keyword in this context indicates the function has requirements we
    // need to uphold when we call this function, because Rust can’t guarantee we’ve
    // met these requirements. By calling an unsafe function within an unsafe block,
    // we’re saying that we’ve read this function’s documentation and take
    // responsibility for upholding the function’s contracts.
    unsafe {
        dangerous();
    }

    // Creating a Safe Abstraction over Unsafe Code
    // Just because a function contains unsafe code doesn’t mean we need to mark
    // the entire function as unsafe. In fact, wrapping unsafe code in a safe
    // function is a common abstraction. As an example, let’s study a function
    // from the standard library, split_at_mut, that requires some unsafe code
    // and explore how we might implement it. This safe method is defined on
    // mutable slices: it takes one slice and makes it two by splitting the
    // slice at the index given as an argument.
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // Note that we don’t need to mark the resulting split_at_mut function as
    // unsafe, and we can call this function from safe Rust. We’ve created a
    // safe abstraction to the unsafe code with an implementation of the
    // function that uses unsafe code in a safe way, because it creates only
    // valid pointers from the data this function has access to.

    // In contrast, the use of slice::from_raw_parts_mut would likely crash when
    // the slice is used. This code takes an arbitrary memory location and
    // creates a slice 10,000 items long.
    let address = 0x01234usize;
    let r = address as *mut i32;
    // We don’t own the memory at this arbitrary location, and there is no
    // guarantee that the slice this code creates contains valid i32 values.
    // Attempting to use values as though it’s a valid slice results in
    // undefined behavior.
    let _values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    // Using extern Functions to Call External Code
    // Sometimes, your Rust code might need to interact with code written in
    // another language. For this, Rust has a keyword, extern, that facilitates
    // the creation and use of a Foreign Function Interface (FFI). An FFI is a
    // way for a programming language to define functions and enable a different
    // (foreign) programming language to call those functions.
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // Accessing or Modifying a Mutable Static Variable
    // Until now, we’ve not talked about global variables, which Rust does
    // support but can be problematic with Rust’s ownership rules. If two
    // threads are accessing the same mutable global variable, it can cause a
    // data race.
    println!("name is: {}", HELLO_WORLD);

    // Constants and immutable static variables might seem similar, but a subtle
    // difference is that values in a static variable have a fixed address in
    // memory. Using the value will always access the same data. Constants, on
    // the other hand, are allowed to duplicate their data whenever they’re
    // used.

    // Another difference between constants and static variables is that static
    // variables can be mutable. Accessing and modifying mutable static
    // variables is unsafe.
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    // As with regular variables, we specify mutability using the mut keyword.
    // Any code that reads or writes from COUNTER must be within an unsafe
    // block. This code compiles and prints COUNTER: 3 as we would expect
    // because it’s single threaded. Having multiple threads access COUNTER
    // would likely result in data races.

    // With mutable data that is globally accessible, it’s difficult to ensure
    // there are no data races, which is why Rust considers mutable static
    // variables to be unsafe. Where possible, it’s preferable to use the
    // concurrency techniques and thread-safe smart pointers we discussed in
    // Chapter 16 so the compiler checks that data accessed from different
    // threads is done safely.

    // Implementing an Unsafe Trait
    // Another use case for unsafe is implementing an unsafe trait. A trait is
    // unsafe when at least one of its methods has some invariant that the
    // compiler can’t verify. We can declare that a trait is unsafe by adding
    // the unsafe keyword before trait and marking the implementation of the
    // trait as unsafe too.

    // Accessing Fields of a Union
    // The final action that works only with unsafe is accessing fields of a
    // union. A union is similar to a struct, but only one declared field is
    // used in a particular instance at one time. Unions are primarily used to
    // interface with unions in C code. Accessing union fields is unsafe because
    // Rust can’t guarantee the type of the data currently being stored in the
    // union instance. You can learn more about unions in the Rust Reference.

    // When to Use Unsafe Code
    // Using unsafe to take one of the five actions (superpowers) just discussed
    // isn’t wrong or even frowned upon. But it is trickier to get unsafe code
    // correct because the compiler can’t help uphold memory safety. When you
    // have a reason to use unsafe code, you can do so, and having the explicit
    // unsafe annotation makes it easier to track down the source of problems
    // when they occur.
}

unsafe fn dangerous() {}

// Within the extern "C" block, we list the names and signatures of external
// functions from another language we want to call. The "C" part defines which
// application binary interface (ABI) the external function uses: the ABI
// defines how to call the function at the assembly level. The "C" ABI is the
// most common and follows the C programming language’s ABI.
extern "C" {
    fn abs(input: i32) -> i32;
}

// Calling Rust Functions from Other Languages
// We can also use extern to create an interface that allows other languages to
// call Rust functions. Instead of an extern block, we add the extern keyword
// and specify the ABI to use just before the fn keyword. We also need to add a
// #[no_mangle] annotation to tell the Rust compiler not to mangle the name of
// this function. Mangling is when a compiler changes the name we’ve given a
// function to a different name that contains more information for other parts
// of the compilation process to consume but is less human readable. Every
// programming language compiler mangles names slightly differently, so for a
// Rust function to be nameable by other languages, we must disable the Rust
// compiler’s name mangling.

// In the following example, we make the call_from_c function accessible from C
// code, after it’s compiled to a shared library and linked from C:
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// Static variables can only store references with the 'static lifetime, which
// means the Rust compiler can figure out the lifetime and we aren’t required to
// annotate it explicitly. Accessing an immutable static variable is safe.
static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: u32 = 0;
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe trait Foo {
    // methods go here
}

// By using unsafe impl, we’re promising that we’ll uphold the invariants that
// the compiler can’t verify.
unsafe impl Foo for i32 {
    // method implementations go here
}

// If we implement a type that contains a type that is not Send or Sync, such as
// raw pointers, and we want to mark that type as Send or Sync, we must use
// unsafe. Rust can’t verify that our type upholds the guarantees that it can be
// safely sent across threads or accessed from multiple threads; therefore, we
// need to do those checks manually and indicate as such with unsafe.
