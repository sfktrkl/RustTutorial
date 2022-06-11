use std::error::Error;
use std::fs;

// Splitting Code into a Library Crate
// Our minigrep project is looking good so far! Now we’ll split the src/main.rs
// file and put some code into the src/lib.rs file so we can test it and have a
// src/main.rs file with fewer responsibilities.
// Let’s move all the code that isn’t the main function from src/main.rs to src/lib.rs:
//  The run function definition
//  The relevant use statements
//  The definition of Config
//  The Config::new function definition

pub fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}

// We’ve added a struct named Config defined to have fields named query and filename.
// The signature of parse_config now indicates that it returns a Config value.
// In the body of parse_config, where we used to return string slices that
// reference String values in args, we now define Config to contain owned String
// values. The args variable in main is the owner of the argument values and is
// only letting the parse_config function borrow them, which means we’d violate
// Rust’s borrowing rules if Config tried to take ownership of the values in args.
pub struct Config {
    pub query: String,
    pub filename: String,
}

pub fn parse_config2(args: &[String]) -> Config {
    // The Trade-Offs of Using clone
    // There’s a tendency among many Rustaceans to avoid using clone to fix
    // ownership problems because of its runtime cost.
    // But for now, it’s okay to copy a few strings to continue making progress
    // because you’ll make these copies only once and your filename and query
    // string are very small. It’s better to have a working program that’s
    // a bit inefficient than to try to hyperoptimize code on your first pass.
    // As you become more experienced with Rust, it’ll be easier to start with
    // the most efficient solution, but for now, it’s perfectly acceptable to
    // call clone.
    let query = args[1].clone();
    let filename = args[2].clone();
    Config { query, filename }
}

// Creating a Constructor for Config
// So far, we’ve extracted the logic responsible for parsing the command line
// arguments from main and placed it in the parse_config function. Doing so
// helped us to see that the query and filename values were related and that
// relationship should be conveyed in our code. We then added a Config struct
// to name the related purpose of query and filename and to be able to return
// the values’ names as struct field names from the parse_config function.
impl Config {
    pub fn new(args: &[String]) -> Config {
        // Fixing the Error Handling
        // ow we’ll work on fixing our error handling. Recall that attempting
        // to access the values in the args vector at index 1 or index 2 will
        // cause the program to panic if the vector contains fewer than three items.
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename }
    }
}

impl Config {
    // Returning a Result from new Instead of Calling panic!
    // We can instead return a Result value that will contain a Config instance
    // in the successful case and will describe the problem in the error case.
    // When Config::new is communicating to main, we can use the Result type to
    // signal there was a problem. Then we can change main to convert an Err
    // variant into a more practical error for our users without the surrounding
    // text about thread 'main' and RUST_BACKTRACE that a call to panic! causes.
    pub fn new2(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // We’ve made two changes in the body of the new function: instead
            // of calling panic! when the user doesn’t pass enough arguments,
            // we now return an Err value.
            // Returning an Err value from Config::new allows the main function
            // to handle the Result value returned from the new function and exit
            // the process more cleanly in the error case.
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        // We’ve wrapped the Config return value in an Ok. These changes make
        // the function conform to its new type signature.
        Ok(Config { query, filename })
    }
}

// Extracting Logic from main
// Now that we’ve finished refactoring the configuration parsing, let’s turn to
// the program’s logic. As we stated in “Separation of Concerns for Binary Projects”,
// we’ll extract a function named run that will hold all the logic currently in
// the main function that isn’t involved with setting up configuration or handling
// errors. When we’re done, main will be concise and easy to verify by inspection,
// and we’ll be able to write tests for all the other logic.
pub fn run(config: Config) {
    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    println!("With text:\n{}", contents);
}

// Returning Errors from the run Function
// Instead of allowing the program to panic by calling expect, the run function
// will return a Result<T, E> when something goes wrong. This will let us
// further consolidate into main the logic around handling errors in a user
//-friendly way.
pub fn run2(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
    Ok(())
}
