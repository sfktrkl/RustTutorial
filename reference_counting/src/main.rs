use std::rc::Rc;
// Rc<T>, the Reference Counted Smart Pointer

// In the majority of cases, ownership is clear: you know exactly which variable
// owns a given value. However, there are cases when a single value might have
// multiple owners. For example, in graph data structures, multiple edges might
// point to the same node, and that node is conceptually owned by all of the
// edges that point to it. A node shouldn’t be cleaned up unless it doesn’t have
// any edges pointing to it and so has no owners.

// You have to enable multiple ownership explicitly by using the Rust type Rc<T>,
// which is an abbreviation for reference counting. The Rc<T> type keeps track
// of the number of references to a value to determine whether or not the value
// is still in use. If there are zero references to a value, the value can be
// cleaned up without any references becoming invalid.

// We use the Rc<T> type when we want to allocate some data on the heap for
// multiple parts of our program to read and we can’t determine at compile time
// which part will finish using the data last. If we knew which part would finish
// last, we could just make that part the data’s owner, and the normal ownership
// rules enforced at compile time would take effect.

// Note that Rc<T> is only for use in single-threaded scenarios.
use crate::List::{Cons, Nil};
// Each Cons variant will now hold a value and an Rc<T> pointing to a List. When
// we create b, instead of taking ownership of a, we’ll clone the Rc<List> that
// a is holding, thereby increasing the number of references from one to two and
// letting a and b share ownership of the data in that Rc<List>.

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    // Using Rc<T> to Share Data
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // We’ll also clone a when creating c, increasing the number of references
    // from two to three. Every time we call Rc::clone, the reference count to
    // the data within the Rc<List> will increase, and the data won’t be cleaned
    // up unless there are zero references to it.

    // We could have called a.clone() rather than Rc::clone(&a), but Rust’s
    // convention is to use Rc::clone in this case. The implementation of
    // Rc::clone doesn’t make a deep copy of all the data like most types’
    // implementations of clone do. The call to Rc::clone only increments the
    // reference count, which doesn’t take much time. Deep copies of data can
    // take a lot of time. By using Rc::clone for reference counting, we can
    // visually distinguish between the deep-copy kinds of clones and the kinds
    // of clones that increase the reference count. When looking for performance
    // problems in the code, we only need to consider the deep-copy clones and
    // can disregard calls to Rc::clone.
    let _b = Cons(3, Rc::clone(&a));
    let _c = Cons(4, Rc::clone(&a));

    // Cloning an Rc<T> Increases the Reference Count
    // At each point in the program where the reference count changes, we print
    // the reference count, which we get by calling the Rc::strong_count function.
    // This function is named strong_count rather than count because the Rc<T>
    // type also has a weak_count.

    // We can see that the Rc<List> in a has an initial reference count of 1;
    // then each time we call clone, the count goes up by 1. When c goes out of
    // scope, the count goes down by 1. We don’t have to call a function to
    // decrease the reference count like we have to call Rc::clone to increase
    // the reference count: the implementation of the Drop trait decreases the
    // reference count automatically when an Rc<T> value goes out of scope.

    // What we can’t see in this example is that when b and then a go out of
    // scope at the end of main, the count is then 0, and the Rc<List> is
    // cleaned up completely. Using Rc<T> allows a single value to have
    // multiple owners, and the count ensures that the value remains valid
    // as long as any of the owners still exist.
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
