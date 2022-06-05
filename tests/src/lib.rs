#![allow(dead_code)]

// Controlling How Tests Are Run
// Just as cargo run compiles your code and then runs the resulting binary,
// cargo test compiles your code in test mode and runs the resulting test binary.
// You can specify command line options to change the default behavior of cargo
// test. For example, the default behavior of the binary produced by cargo test
// is to run all the tests in parallel and capture output generated during test
// runs, preventing the output from being displayed and making it easier to read
// the output related to the test results.

// Running Tests in Parallel or Consecutively
// When you run multiple tests, by default they run in parallel using threads.
// This means the tests will finish running faster so you can get feedback
// quicker on whether or not your code is working. Because the tests are running
// at the same time, make sure your tests don’t depend on each other or on any
// shared state, including a shared environment, such as the current working
// directory or environment variables.
// If you don’t want to run the tests in parallel or if you want more fine-grained
// control over the number of threads used, you can send the --test-threads flag
// and the number of threads you want to use to the test binary. Take a look
// at the following example:
//$ cargo test -- --test-threads=1

fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    // Showing Function Output
    // By default, if a test passes, Rust’s test library captures anything
    // printed to standard output. For example, if we call println! in a test
    // and the test passes, we won’t see the println! output in the terminal;
    // we’ll see only the line that indicates the test passed. If a test fails,
    // we’ll see whatever was printed to standard output with the rest of the
    // failure message.

    // Note that nowhere in this output do we see I got the value 4, which is
    // what is printed when the test that passes runs. That output has been
    // captured. The output from the test that failed, I got the value 8,
    // appears in the section of the test summary output, which also shows
    // the cause of the test failure.
    // If we want to see printed values for passing tests as well, we can tell
    // Rust to also show the output of successful tests at the end with
    // --show-output.
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    // Running a Subset of Tests by Name
    // Sometimes, running a full test suite can take a long time. If you’re
    // working on code in a particular area, you might want to run only the
    // tests pertaining to that code. You can choose which tests to run by
    // passing cargo test the name or names of the test(s) you want to run
    // as an argument.
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }

    // Running Single Tests
    // We can pass the name of any test function to cargo test to run only that test.
    //$ cargo test one_hundred

    // Filtering to Run Multiple Tests
    // We can specify part of a test name, and any test whose name matches that value will be run.
    //$ cargo test add

    // Ignoring Some Tests Unless Specifically Requested
    // Sometimes a few specific tests can be very time-consuming to execute,
    // so you might want to exclude them during most runs of cargo test.
    // Rather than listing as arguments all tests you do want to run, you can
    // instead annotate the time-consuming tests using the ignore attribute
    // to exclude them.
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // By controlling which tests run, you can make sure your cargo test results
    // will be fast. When you’re at a point where it makes sense to check the
    // results of the ignored tests and you have time to wait for the results,
    // you can run cargo test -- --ignored instead. If you want to run all tests
    // whether they’re ignored or not, you can run cargo test -- --include-ignored.
    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}
