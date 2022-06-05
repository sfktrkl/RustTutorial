// Integration Tests
// In Rust, integration tests are entirely external to your library. They use
// your library in the same way any other code would, which means they can only
// call functions that are part of your library’s public API. Their purpose is
// to test whether many parts of your library work together correctly. Units of
// code that work correctly on their own could have problems when integrated,
// so test coverage of the integrated code is important as well. To create
// integration tests, you first need a tests directory.

// The tests Directory
// We create a tests directory at the top level of our project directory,
// next to src. Cargo knows to look for integration test files in this directory.
// We can then make as many test files as we want to in this directory, and Cargo
// will compile each of the files as an individual crate.

// We’ve added use test_organization at the top of the code, which we didn’t
// need in the unit tests. The reason is that each file in the tests directory
// is a separate crate, so we need to bring our library into each test crate’s scope.
use test_organization;

// We don’t need to annotate any code in tests/integration_test.rs with #[cfg(test)].
// Cargo treats the tests directory specially and compiles files in this directory
// only when we run cargo test.
// The three sections of output include the unit tests, the integration test,
// and the doc tests. The first section for the unit tests is the same as we’ve
// been seeing: one line for each unit test and then a summary line for the unit tests.
// We can still run a particular integration test function by specifying the test
// function’s name as an argument to cargo test. To run all the tests in
// a particular integration test file, use the --test argument of cargo test
// followed by the name of the file.
//$ cargo test --test integration_test
#[test]
fn it_adds_two() {
    assert_eq!(4, test_organization::add_two(2));
}

// After we’ve created tests/common/mod.rs, we can use it from any of the
// integration test files as a module.
mod common;

#[test]
fn it_adds_two2() {
    common::setup();
    assert_eq!(4, test_organization::add_two(2));
}

// Integration Tests for Binary Crates
// If our project is a binary crate that only contains a src/main.rs file and
// doesn’t have a src/lib.rs file, we can’t create integration tests in the
// tests directory and bring functions defined in the src/main.rs file into
// scope with a use statement. Only library crates expose functions that other
// crates can use; binary crates are meant to be run on their own.

// This is one of the reasons Rust projects that provide a binary have
// a straightforward src/main.rs file that calls logic that lives in the
// src/lib.rs file. Using that structure, integration tests can test the
// library crate with use to make the important functionality available.
// If the important functionality works, the small amount of code in the
// src/main.rs file will work as well, and that small amount of code doesn’t
// need to be tested.
