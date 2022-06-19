pub fn add_one(x: i32) -> i32 {
    x + 1
}

// Adding a Test to a Workspace
// For another enhancement, let’s add a test of the add_one::add_one function
// within the add_one crate:
// Now run cargo test in the top-level add directory.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}