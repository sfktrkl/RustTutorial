use std::env;
use std::fs;

fn main() {
    // Reading the Argument Values
    // To enable minigrep to read the values of command line arguments we pass
    // to it, we’ll need a function provided in Rust’s standard library, which
    // is std::env::args. This function returns an iterator of the command line
    // arguments that were given to minigrep.
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    // Saving the Argument Values in Variables
    // Printing the value of the vector of arguments illustrated that
    // the program is able to access the values specified as command line
    // arguments. Now we need to save the values of the two arguments in
    // variables so we can use the values throughout the rest of the program.
    let query = &args[1];
    let filename = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", filename);
    //$ cargo run needle haystack

    // Reading a File
    // Now we’ll add functionality to read the file that is specified in the
    // filename command line argument. First, we need a sample file to test
    // it with: the best kind of file to use to make sure minigrep is working
    // is one with a small amount of text over multiple lines with some repeated words.
    println!("In file {}", filename);
    // In main, we’ve added a new statement: fs::read_to_string takes the
    // filename, opens that file, and returns a Result<String> of the file’s contents.
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    // After that statement, we’ve again added a temporary println! statement
    // that prints the value of contents after the file is read, so we can
    // check that the program is working so far.
    println!("With text:\n{}", contents);
    //$ cargo run the poem.txt

    // Great! The code read and then printed the contents of the file. But the
    // code has a few flaws. The main function has multiple responsibilities:
    // generally, functions are clearer and easier to maintain if each function
    // is responsible for only one idea. The other problem is that we’re not
    // handling errors as well as we could. The program is still small, so these
    // flaws aren’t a big problem, but as the program grows, it will be harder
    // to fix them cleanly. It’s good practice to begin refactoring early on
    // when developing a program, because it’s much easier to refactor smaller
    // amounts of code. We’ll do that next.
}
