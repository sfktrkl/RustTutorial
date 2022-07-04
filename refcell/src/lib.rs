#![allow(dead_code)]
// A Use Case for Interior Mutability: Mock Objects
// Sometimes during testing a programmer will use a type in place of another type,
// in order to observe particular behavior and assert it's implemented correctly.
// This placeholder type is called a test double. Think of it in the sense of
// a "stunt double" in filmmaking, where a person steps in and substitutes for
// an actor to do a particular tricky scene. Test doubles stand in for other types
// when we're running tests. Mock objects are specific types of test doubles
// that record what happens during a test so you can assert that the correct
// actions took place.

// Rust doesn’t have objects in the same sense as other languages have objects,
// and Rust doesn’t have mock object functionality built into the standard
// library as some other languages do. However, you can definitely create a
// struct that will serve the same purposes as a mock object.

// Here’s the scenario we’ll test: we’ll create a library that tracks a value
// against a maximum value and sends messages based on how close to the maximum
// value the current value is. This library could be used to keep track of
// a user’s quota for the number of API calls they’re allowed to make, for example.

// Our library will only provide the functionality of tracking how close to the
// maximum a value is and what the messages should be at what times. Applications
// that use our library will be expected to provide the mechanism for sending the
// messages: the application could put a message in the application, send an email,
// send a text message, or something else. The library doesn’t need to know that
// detail. All it needs is something that implements a trait we’ll provide called
// Messenger.
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

// One important part of this code is that the Messenger trait has one method
// called send that takes an immutable reference to self and the text of the
// message. This trait is the interface our mock object needs to implement so
// that the mock can be used in the same way a real object is. The other
// important part is that we want to test the behavior of the set_value method
// on the LimitTracker. We can change what we pass in for the value parameter,
// but set_value doesn’t return anything for us to make assertions on. We want
// to be able to say that if we create a LimitTracker with something that
// implements the Messenger trait and a particular value for max, when we pass
// different numbers for value, the messenger is told to send the appropriate messages.

// We need a mock object that, instead of sending an email or text message when
// we call send, will only keep track of the messages it’s told to send. We can
// create a new instance of the mock object, create a LimitTracker that uses the
// mock object, call the set_value method on LimitTracker, and then check that
// the mock object has the messages we expect.
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    // This test code defines a MockMessenger struct that has a sent_messages
    // field with a Vec of String values to keep track of the messages it’s told
    // to send. We also define an associated function new to make it convenient
    // to create new MockMessenger values that start with an empty list of
    // messages. We then implement the Messenger trait for MockMessenger so we
    // can give a MockMessenger to a LimitTracker. In the definition of the send
    // method, we take the message passed in as a parameter and store it in the
    // MockMessenger list of sent_messages.
    impl Messenger for MockMessenger {
        // We can’t modify the MockMessenger to keep track of the messages,
        // because the send method takes an immutable reference to self. We
        // also can’t take the suggestion from the error text to use &mut self
        // instead, because then the signature of send wouldn’t match the
        // signature in the Messenger trait definition.
        // This is a situation in which interior mutability can help! We’ll store
        // the sent_messages within a RefCell<T>, and then the send method will
        // be able to modify sent_messages to store the messages we’ve seen.
        fn send(&self, _message: &str) {
            //self.sent_messages.push(String::from(message));
        }
    }

    struct MockMessenger2 {
        // The sent_messages field is now of type RefCell<Vec<String>> instead
        // of Vec<String>. In the new function, we create a new RefCell<Vec<String>>
        // instance around the empty vector.
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger2 {
        fn new() -> MockMessenger2 {
            MockMessenger2 {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger2 {
        // For the implementation of the send method, the first parameter is
        // still an immutable borrow of self, which matches the trait definition.
        // We call borrow_mut on the RefCell<Vec<String>> in self.sent_messages
        // to get a mutable reference to the value inside the RefCell<Vec<String>>,
        // which is the vector. Then we can call push on the mutable reference
        // to the vector to keep track of the messages sent during the test.
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));

            // Keeping Track of Borrows at Runtime with RefCell<T>
            // When creating immutable and mutable references, we use the & and
            // &mut syntax, respectively. With RefCell<T>, we use the borrow
            // and borrow_mut methods, which are part of the safe API that
            // belongs to RefCell<T>. The borrow method returns the smart pointer
            // type Ref<T>, and borrow_mut returns the smart pointer type
            // RefMut<T>. Both types implement Deref, so we can treat them like
            // regular references.

            // The RefCell<T> keeps track of how many Ref<T> and RefMut<T> smart
            // pointers are currently active. Every time we call borrow, the
            // RefCell<T> increases its count of how many immutable borrows are
            // active. When a Ref<T> value goes out of scope, the count of
            // immutable borrows goes down by one. Just like the compile-time
            // borrowing rules, RefCell<T> lets us have many immutable borrows
            // or one mutable borrow at any point in time.

            // If we try to violate these rules, rather than getting a compiler
            // error as we would with references, the implementation of RefCell<T>
            // will panic at runtime.
            //let mut borrow_again = self.sent_messages.borrow_mut();
            //borrow_again.push(String::from(message));

            // Notice that the code panicked with the message already borrowed:
            // BorrowMutError. This is how RefCell<T> handles violations of the
            // borrowing rules at runtime.

            // Choosing to catch borrowing errors at runtime rather than compile
            // time, as we've done here, means you'd potentially be finding
            // mistakes in your code later in the development process: possibly
            // not until your code was deployed to production. Also, your code
            // would incur a small runtime performance penalty as a result of
            // keeping track of the borrows at runtime rather than compile time.
            // However, using RefCell<T> makes it possible to write a mock object
            // that can modify itself to keep track of the messages it has seen
            // while you’re using it in a context where only immutable values
            // are allowed. You can use RefCell<T> despite its trade-offs to get
            // more functionality than regular references provide.
        }
    }

    // In the test, we’re testing what happens when the LimitTracker is told to
    // set value to something that is more than 75 percent of the max value.
    // First, we create a new MockMessenger, which will start with an empty list
    // of messages. Then we create a new LimitTracker and give it a reference
    // to the new MockMessenger and a max value of 100. We call the set_value
    // method on the LimitTracker with a value of 80, which is more than 75
    // percent of 100.
    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger2::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);

        // The last change we have to make is in the assertion: to see how many
        // items are in the inner vector, we call borrow on the
        // RefCell<Vec<String>> to get an immutable reference to the vector.
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
