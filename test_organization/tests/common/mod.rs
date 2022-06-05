// Submodules in Integration Tests
// As you add more integration tests, you might want to make more than one
// file in the tests directory to help organize them; for example, you can
// group the test functions by the functionality they’re testing. As mentioned
// earlier, each file in the tests directory is compiled as its own separate crate.

// Treating each integration test file as its own crate is useful to create
// separate scopes that are more like the way end users will be using your crate.
// However, this means files in the tests directory don’t share the same behavior
// as files in src do.
// When we run the tests again, we’ll see a new section in the test output for
// the common.rs file, even though this file doesn’t contain any test functions
// nor did we call the setup function from anywhere.

// Having common appear in the test results with running 0 tests displayed for
// it is not what we wanted. We just wanted to share some code with the other
// integration test files.
// To avoid having common appear in the test output, instead of creating tests/common.rs,
// we’ll create tests/common/mod.rs. This is an alternate naming convention that
// Rust also understands. Naming the file this way tells Rust not to treat the
// common module as an integration test file.
pub fn setup() {
    // setup code specific to your library's tests would go here
}