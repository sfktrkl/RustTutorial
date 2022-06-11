use std::env;

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
}
