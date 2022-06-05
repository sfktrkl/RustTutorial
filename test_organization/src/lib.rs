
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

// Test Organization
// As mentioned at the start of the chapter, testing is a complex discipline,
// and different people use different terminology and organization. The Rust
// community thinks about tests in terms of two main categories: unit tests
// and integration tests. Unit tests are small and more focused, testing one
// module in isolation at a time, and can test private interfaces. Integration
// tests are entirely external to your library and use your code in the same
// way any other external code would, using only the public interface and
// potentially exercising multiple modules per test.

// Unit Tests
// The purpose of unit tests is to test each unit of code in isolation from the
// rest of the code to quickly pinpoint where code is and isn’t working as expected.
// You’ll put unit tests in the src directory in each file with the code that
// they’re testing. The convention is to create a module named tests in each
// file to contain the test functions and to annotate the module with cfg(test).

// The Tests Module and #[cfg(test)]
// The #[cfg(test)] annotation on the tests module tells Rust to compile and
// run the test code only when you run cargo test, not when you run cargo build.
// This saves compile time when you only want to build the library and saves
// space in the resulting compiled artifact because the tests are not included.
// You’ll see that because integration tests go in a different directory, they
// don’t need the #[cfg(test)] annotation. However, because unit tests go in the
// same files as the code, you’ll use #[cfg(test)] to specify that they shouldn’t
// be included in the compiled result.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    // Testing Private Functions
    // There’s debate within the testing community about whether or not private
    // functions should be tested directly, and other languages make it difficult
    // or impossible to test private functions. Regardless of which testing ideology
    // you adhere to, Rust’s privacy rules do allow you to test private functions.
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
