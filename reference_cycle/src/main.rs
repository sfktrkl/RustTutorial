// Reference Cycles Can Leak Memory
// Rust’s memory safety guarantees make it difficult, but not impossible, to
// accidentally create memory that is never cleaned up (known as a memory leak).
// Preventing memory leaks entirely is not one of Rust’s guarantees, meaning
// memory leaks are memory safe in Rust. We can see that Rust allows memory
// leaks by using Rc<T> and RefCell<T>: it’s possible to create references where
// items refer to each other in a cycle. This creates memory leaks because the
// reference count of each item in the cycle will never reach 0, and the values
// will never be dropped.

use crate::List::{Cons, Nil};
use std::rc:: {Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    // We create an Rc<List> instance holding a List value in the variable a
    // with an initial list of 5, Nil. We then create an Rc<List> instance
    // holding another List value in the variable b that contains the value 10
    // and points to the list in a.
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    // We modify a so it points to b instead of Nil, creating a cycle.  We do
    // that by using the tail method to get a reference to the RefCell<Rc<List>>
    // in a, which we put in the variable link. Then we use the borrow_mut
    // method on the RefCell<Rc<List>> to change the value inside from an
    // Rc<List> that holds a Nil value to the Rc<List> in b.
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    // The reference count of the Rc<List> instances in both a and b are 2 after
    // we change the list in a to point to b. At the end of main, Rust drops the
    // variable b, which decreases the reference count of the Rc<List> instance
    // from 2 to 1. The memory that Rc<List> has on the heap won’t be dropped at
    // this point, because its reference count is 1, not 0. Then Rust drops a,
    // which decreases the reference count of the a Rc<List> instance from 2 to
    // 1 as well. This instance’s memory can’t be dropped either, because the
    // other Rc<List> instance still refers to it. The memory allocated to the
    // list will remain uncollected forever.
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    //println!("a next item = {:?}", a.tail());

    // Preventing Reference Cycles: Turning an Rc<T> into a Weak<T>
    // So far, we’ve demonstrated that calling Rc::clone increases the strong_count
    // of an Rc<T> instance, and an Rc<T> instance is only cleaned up if its
    // strong_count is 0. You can also create a weak reference to the value within
    // an Rc<T> instance by calling Rc::downgrade and passing a reference to the
    // Rc<T>. Strong references are how you can share ownership of an Rc<T>
    // instance. Weak references don’t express an ownership relationship, and their
    // count doesn't affect when an Rc<T> instance is cleaned up. They won’t cause a
    // reference cycle because any cycle involving some weak references will be
    // broken once the strong reference count of values involved is 0.

    // When you call Rc::downgrade, you get a smart pointer of type Weak<T>. Instead
    // of increasing the strong_count in the Rc<T> instance by 1, calling
    // Rc::downgrade increases the weak_count by 1. The Rc<T> type uses weak_count
    // to keep track of how many Weak<T> references exist, similar to strong_count.
    // The difference is the weak_count doesn’t need to be 0 for the Rc<T> instance
    // to be cleaned up.

    // Because the value that Weak<T> references might have been dropped, to do
    // anything with the value that a Weak<T> is pointing to, you must make sure the
    // value still exists. Do this by calling the upgrade method on a Weak<T>
    // instance, which will return an Option<Rc<T>>. You’ll get a result of Some if
    // the Rc<T> value has not been dropped yet and a result of None if the Rc<T>
    // value has been dropped. Because upgrade returns an Option<Rc<T>>, Rust will
    // ensure that the Some case and the None case are handled, and there won’t be
    // an invalid pointer.

    // As an example, rather than using a list whose items know only about the next
    // item, we’ll create a tree whose items know about their children items and
    // their parent items.

    // Creating the leaf node with the exception of the parent field: leaf
    // starts out without a parent, so we create a new, empty Weak<Node>
    // reference instance.
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    // At this point, when we try to get a reference to the parent of leaf by
    // using the upgrade method, we get a None value.
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // When we create the branch node, it will also have a new Weak<Node>
    // reference in the parent field, because branch doesn’t have a parent node.
    // We still have leaf as one of the children of branch. Once we have the
    // Node instance in branch, we can modify leaf to give it a Weak<Node>
    // reference to its parent. We use the borrow_mut method on the
    // RefCell<Weak<Node>> in the parent field of leaf, and then we use the
    // Rc::downgrade function to create a Weak<Node> reference to branch from
    // the Rc<Node> in branch.
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    // When we print the parent of leaf again, this time we’ll get a Some
    // variant holding branch: now leaf can access its parent!
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // Visualizing Changes to strong_count and weak_count
    // Let’s look at how the strong_count and weak_count values of the Rc<Node>
    // instances change by creating a new inner scope and moving the creation of
    // branch into that scope. By doing so, we can see what happens when branch
    // is created and then dropped when it goes out of scope.
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // After leaf is created, its Rc<Node> has a strong count of 1 and a weak
    // count of 0.
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        // In the inner scope, we create branch and associate it with leaf, at
        // which point when we print the counts, the Rc<Node> in branch will
        // have a strong count of 1 and a weak count of 1 (for leaf.parent
        // pointing to branch with a Weak<Node>).
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        // When we print the counts in leaf, we’ll see it will have a strong
        // count of 2, because branch now has a clone of the Rc<Node> of leaf
        // stored in branch.children, but will still have a weak count of 0.
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    // When the inner scope ends, branch goes out of scope and the strong count
    // of the Rc<Node> decreases to 0, so its Node is dropped. The weak count of
    // 1 from leaf.parent has no bearing on whether or not Node is dropped, so
    // we don’t get any memory leaks!
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    // If we try to access the parent of leaf after the end of the scope, we’ll
    // get None again. At the end of the program, the Rc<Node> in leaf has a
    // strong count of 1 and a weak count of 0, because the variable leaf is now
    // the only reference to the Rc<Node> again.
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    // All of the logic that manages the counts and value dropping is built into
    // Rc<T> and Weak<T> and their implementations of the Drop trait. By
    // specifying that the relationship from a child to its parent should be a
    // Weak<T> reference in the definition of Node, you’re able to have parent
    // nodes point to child nodes and vice versa without creating a reference
    // cycle and memory leaks.
}

// Creating a Tree Data Structure: a Node with Child Nodes
// We want a Node to own its children, and we want to share that ownership with
// variables so we can access each Node in the tree directly. To do this, we
// define the Vec<T> items to be values of type Rc<Node>. We also want to modify
// which nodes are children of another node, so we have a RefCell<T> in children
// around the Vec<Rc<Node>>.
#[derive(Debug)]
struct Node {
    value: i32,
    // We clone the Rc<Node> in leaf and store that in branch, meaning the Node
    // in leaf now has two owners: leaf and branch. We can get from branch to
    // leaf through branch.children, but there’s no way to get from leaf to
    // branch. The reason is that leaf has no reference to branch and doesn’t
    // know they’re related. We want leaf to know that branch is its parent.
    children: RefCell<Vec<Rc<Node>>>,
    // Adding a Reference from a Child to Its Parent
    // To make the child node aware of its parent, we need to add a parent field
    // to our Node struct definition. The trouble is in deciding what the type
    // of parent should be. We know it can’t contain an Rc<T>, because that
    // would create a reference cycle with leaf.parent pointing to branch and
    // branch.children pointing to leaf, which would cause their strong_count
    // values to never be 0.

    // Thinking about the relationships another way, a parent node should own
    // its children: if a parent node is dropped, its child nodes should be
    // dropped as well. However, a child should not own its parent: if we drop a
    // child node, the parent should still exist. This is a case for weak
    // references!

    // So instead of Rc<T>, we’ll make the type of parent use Weak<T>,
    // specifically a RefCell<Weak<Node>>.
    parent: RefCell<Weak<Node>>,
}
