use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

// RefCell<T> and the Interior Mutability Pattern

// Interior mutability is a design pattern in Rust that allows you to mutate
// data even when there are immutable references to that data; normally,
// this action is disallowed by the borrowing rules. To mutate data, the pattern
// uses unsafe code inside a data structure to bend Rust’s usual rules that
// govern mutation and borrowing. We haven’t yet covered unsafe code that
// indicates we're checking the rules manually instead of the compiler checking
// them for us. We can use types that use the interior mutability pattern only
// when we can ensure that the borrowing rules will be followed at runtime,
// even though the compiler can’t guarantee that. The unsafe code involved is
// then wrapped in a safe API, and the outer type is still immutable.

// Enforcing Borrowing Rules at Runtime with RefCell<T>
// Unlike Rc<T>, the RefCell<T> type represents single ownership over the data
// it holds. So, what makes RefCell<T> different from a type like Box<T>?
//  At any given time, you can have either (but not both) one mutable reference
//      or any number of immutable references.
//  References must always be valid.

// With references and Box<T>, the borrowing rules’ invariants are enforced at
// compile time. With RefCell<T>, these invariants are enforced at runtime.
// With references, if you break these rules, you’ll get a compiler error.
// With RefCell<T>, if you break these rules, your program will panic and exit.

// The advantages of checking the borrowing rules at compile time are that errors
// will be caught sooner in the development process, and there is no impact on
// runtime performance because all the analysis is completed beforehand. For
// those reasons, checking the borrowing rules at compile time is the best
// choice in the majority of cases, which is why this is Rust’s default.

// The advantage of checking the borrowing rules at runtime instead is that
// certain memory-safe scenarios are then allowed, where they would’ve been
// disallowed by the compile-time checks. Static analysis, like the Rust
// compiler, is inherently conservative. Some properties of code are impossible
// to detect by analyzing the code: the most famous example is the Halting Problem,
// which is beyond the scope of this book but is an interesting topic to research.

// Because some analysis is impossible, if the Rust compiler can’t be sure the
// code complies with the ownership rules, it might reject a correct program;
// in this way, it’s conservative. If Rust accepted an incorrect program, users
// wouldn’t be able to trust in the guarantees Rust makes. However, if Rust
// rejects a correct program, the programmer will be inconvenienced, but nothing
// catastrophic can occur. The RefCell<T> type is useful when you’re sure your
// code follows the borrowing rules but the compiler is unable to understand and
// guarantee that.

// Similar to Rc<T>, RefCell<T> is only for use in single-threaded scenarios and
// will give you a compile-time error if you try using it in a multithreaded context.

// Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:
//  Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have
//      single owners.
//  Box<T> allows immutable or mutable borrows checked at compile time; Rc<T>
//      allows only immutable borrows checked at compile time; RefCell<T> allows
//      immutable or mutable borrows checked at runtime.
//  Because RefCell<T> allows mutable borrows checked at runtime, you can mutate
//      the value inside the RefCell<T> even when the RefCell<T> is immutable.

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    // Interior Mutability: A Mutable Borrow to an Immutable Value
    // A consequence of the borrowing rules is that when you have an immutable
    // value, you can’t borrow it mutably.
    let _x = 5;
    // However, there are situations in which it would be useful for a value to
    // mutate itself in its methods but appear immutable to other code. Code
    // outside the value’s methods would not be able to mutate the value.
    // Using RefCell<T> is one way to get the ability to have interior mutability,
    // but RefCell<T> doesn’t get around the borrowing rules completely:
    // the borrow checker in the compiler allows this interior mutability, and
    // the borrowing rules are checked at runtime instead. If you violate the
    // rules, you’ll get a panic! instead of a compiler error.
    //let y = &mut _x;

    // Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>
    // A common way to use RefCell<T> is in combination with Rc<T>. Recall that Rc<T>
    // lets you have multiple owners of some data, but it only gives immutable access
    // to that data. If you have an Rc<T> that holds a RefCell<T>, you can get
    // a value that can have multiple owners and that you can mutate!
    let value = Rc::new(RefCell::new(5));
    // We create a value that is an instance of Rc<RefCell<i32>> and store it in
    // a variable named value so we can access it directly later. Then we create
    // a List in a with a Cons variant that holds value. We need to clone value
    // so both a and value have ownership of the inner 5 value rather than
    // transferring ownership from value to a or having a borrow from value.
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
    // After we’ve created the lists in a, b, and c, we want to add 10 to the
    // value in value. We do this by calling borrow_mut on value, which uses the
    // automatic dereferencing feature to dereference the Rc<T> to the inner
    // RefCell<T> value. The borrow_mut method returns a RefMut<T> smart pointer,
    // and we use the dereference operator on it and change the inner value.
    *value.borrow_mut() += 10;

    // This technique is pretty neat! By using RefCell<T>, we have an outwardly
    // immutable List value. But we can use the methods on RefCell<T> that
    // provide access to its interior mutability so we can modify our data when
    // we need to. The runtime checks of the borrowing rules protect us from
    // data races, and it’s sometimes worth trading a bit of speed for this
    // flexibility in our data structures.
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    // The standard library has other types that provide interior mutability,
    // such as Cell<T>, which is similar except that instead of giving references
    // to the inner value, the value is copied in and out of the Cell<T>. There’s
    // also Mutex<T>, which offers interior mutability that’s safe to use across
    // threads.
}
