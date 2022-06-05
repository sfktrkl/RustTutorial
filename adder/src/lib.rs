#![allow(dead_code)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(_name: &str) -> String {
    format!("Hello")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
}

// At its simplest, a test in Rust is a function that’s annotated with the test
// attribute. Attributes are metadata about pieces of Rust code; one example is
// the derive attribute we used with structs in Chapter 5. To change a function
// into a test function, add #[test] on the line before fn. When you run your
// tests with the cargo test command, Rust builds a test runner binary that runs
// the functions annotated with the test attribute and reports on whether each
// test function passes or fails.
#[cfg(test)]
mod tests {
    // Note the #[test] annotation before the fn line: this attribute indicates
    // this is a test function, so the test runner knows to treat this function
    // as a test. We could also have non-test functions in the tests module to
    // help set up common scenarios or perform common operations, so we need to
    // indicate which functions are tests by using the #[test] attribute.
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    // Let’s add another test, but this time we’ll make a test that fails!
    // Tests fail when something in the test function panics. Each test is run
    // in a new thread, and when the main thread sees that a test thread has
    // died, the test is marked as failed.
    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    // Note that we’ve added a new line inside the tests module: use super::*;.
    // Because the tests module is an inner module, we need to bring the code
    // under test in the outer module into the scope of the inner module.
    // We use a glob here so anything we define in the outer module is available
    // to this tests module.
    use super::*;

    // Checking Results with the assert! Macro
    // The assert! macro, provided by the standard library, is useful when
    // you want to ensure that some condition in a test evaluates to true.
    // We give the assert! macro an argument that evaluates to a Boolean.
    // If the value is true, assert! does nothing and the test passes.
    // If the value is false, the assert! macro calls the panic! macro,
    // which causes the test to fail. Using the assert! macro helps us check
    // that our code is functioning in the way we intend.
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    // Testing Equality with the assert_eq! and assert_ne! Macros
    // A common way to test functionality is to compare the result of the code
    // under test to the value you expect the code to return to make sure
    // they’re equal. You could do this using the assert! macro and passing it
    // an expression using the == operator. However, this is such a common test
    // that the standard library provides a pair of macros—assert_eq! and
    // assert_ne!—to perform this test more conveniently. These macros compare
    // two arguments for equality or inequality, respectively. They’ll also
    // print the two values if the assertion fails, which makes it easier to
    // see why the test failed; conversely, the assert! macro only indicates
    // that it got a false value for the == expression, not the values that
    // led to the false value.
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // Adding Custom Failure Messages
    // You can also add a custom message to be printed with the failure message
    // as optional arguments to the assert!, assert_eq!, and assert_ne! macros.
    // Any arguments specified after the one required argument to assert! or
    // the two required arguments to assert_eq! and assert_ne! are passed along
    // to the format! macro, so you can pass a format string that contains {}
    // placeholders and values to go in those placeholders. Custom messages are
    // useful to document what an assertion means; when a test fails, you’ll
    // have a better idea of what the problem is with the code.
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    // Checking for Panics with should_panic
    // In addition to checking that our code returns the correct values we
    // expect, it’s also important to check that our code handles error
    // conditions as we expect. We can write a test that ensures that
    // attempting to create a Guess instance with a value outside that range
    // panics.
    // We do this by adding another attribute, should_panic, to our test
    // function. This attribute makes a test pass if the code inside the
    // function panics; the test will fail if the code inside the function
    // doesn’t panic.
    #[test]
    // Tests that use should_panic can be imprecise because they only indicate
    // that the code has caused some panic. A should_panic test would pass even
    // if the test panics for a different reason from the one we were expecting
    // to happen. To make should_panic tests more precise, we can add an optional
    // expected parameter to the should_panic attribute.
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    // Using Result<T, E> in Tests
    // So far, we’ve written tests that panic when they fail.
    // We can also write tests that use Result<T, E>!
    // Writing tests so they return a Result<T, E> enables you to use the
    // question mark operator in the body of tests, which can be a convenient
    // way to write tests that should fail if any operation within them returns
    // an Err variant.
    // You can’t use the #[should_panic] annotation on tests that use
    // Result<T, E>. To assert that an operation returns an Err variant,
    // don’t use the question mark operator on the Result<T, E> value. Instead,
    // use assert!(value.is_err()).
    #[test]
    fn it_works2() -> Result<(), String> {
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
