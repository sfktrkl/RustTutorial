use std::env;
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

// Developing the Library’s Functionality with Test-Driven Development
// Now that we’ve extracted the logic into src/lib.rs and left the argument
// collecting and error handling in src/main.rs, it’s much easier to write tests
// for the core functionality of our code. We can call functions directly with
// various arguments and check return values without having to call our binary
// from the command line.

// In this section, we’ll add the searching logic to the minigrep program by
// using the Test-driven development (TDD) process. This software development
// technique follows these steps:
//  Write a test that fails and run it to make sure it fails for the reason you expect.
//  Write or modify just enough code to make the new test pass.
//  Refactor the code you just added or changed and make sure the tests continue to pass.
//  Repeat from step 1!

// Writing a Failing Test
// The test function specifies the behavior we want the search function to have:
// it will take a query and the text to search for the query in, and it will
// return only the lines from the text that contain the query.

#[cfg(test)]
mod tests {
    use super::*;

    // This test searches for the string "duct". The text we’re searching is
    // three lines, only one of which contains "duct" (Note that the backslash
    // after the opening double quote tells Rust not to put a newline character
    // at the beginning of the contents of this string literal). We assert that
    // the value returned from the search function contains only the line we expect.
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    // Working with Environment Variables
    // We’ll improve minigrep by adding an extra feature: an option for case-insensitive
    // searching that the user can turn on via an environment variable. We could make
    // this feature a command line option and require that users enter it each time
    // they want it to apply, but instead we’ll use an environment variable. Doing
    // so allows our users to set the environment variable once and have all their
    // searches be case insensitive in that terminal session.

    // Writing a Failing Test for the Case-Insensitive search Function
    // We want to add a new search_case_insensitive function that we’ll call when
    // the environment variable is on. We’ll continue to follow the TDD process, so
    // the first step is again to write a failing test. We’ll add a new test for the
    // new search_case_insensitive function and rename our old test from one_result
    // to case_sensitive to clarify the differences between the two tests.
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

// Writing Code to Pass the Test
// Currently, our test is failing because we always return an empty vector. To fix that and implement search, our program needs to follow these steps:
// Iterate through each line of the contents.
// Check whether the line contains our query string.
// If it does, add it to the list of values we’re returning.
// If it doesn’t, do nothing.
// Return the list of results that match.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    // Iterating Through Lines with the lines Method
    // Rust has a helpful method to handle line-by-line iteration of strings,
    // conveniently named lines. The lines method returns an iterator.
    for line in contents.lines() {
        // Searching Each Line for the Query
        // Next, we’ll check whether the current line contains our query string.
        // Fortunately, strings have a helpful method named contains that does this for us!
        if line.contains(query) {
            // Storing Matching Lines
            // We also need a way to store the lines that contain our query string.
            // For that, we can make a mutable vector before the for loop and call
            // the push method to store a line in the vector.
            results.push(line);
        }
    }
    results
}

// Using the search Function in the run Function
// Now that the search function is working and tested, we need to call search
// from our run function. We need to pass the config.query value and the contents
// that run reads from the file to the search function.
pub fn run3(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}

// Implementing the search_case_insensitive Function
// The search_case_insensitive function, shown in Listing 12-21, will be almost
// the same as the search function. The only difference is that we’ll lowercase
// the query and each line so whatever the case of the input arguments, they’ll
// be the same case when we check whether the line contains the query.
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        // First, we lowercase the query string and store it in a shadowed variable
        // with the same name. Calling to_lowercase on the query is necessary
        // so no matter whether the user’s query is "rust", "RUST", "Rust", or
        // "rUsT", we’ll treat the query as if it were "rust" and be insensitive
        // to the case. While to_lowercase will handle basic Unicode, it won’t
        // be 100% accurate. If we were writing a real application, we’d want to
        // do a bit more work here, but this section is about environment variables,
        // not Unicode, so we’ll leave it at that here.
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

// Great! They passed. Now, let’s call the new search_case_insensitive function
// from the run function. First, we’ll add a configuration option to the Config
// struct to switch between case-sensitive and case-insensitive search. Adding
// this field will cause compiler errors because we aren’t initializing this
// field anywhere yet:
pub struct Config2 {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

// Note that we added the case_sensitive field that holds a Boolean. Next, we
// need the run function to check the case_sensitive field’s value and use that
// to decide whether to call the search function or the search_case_insensitive function.
pub fn run4(config: Config2) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

// Finally, we need to check for the environment variable. The functions for
// working with environment variables are in the env module in the standard
// library, so we want to bring that module into scope with a use std::env;
// line at the top of src/lib.rs. Then we’ll use the var function from the
// env module to check for an environment variable named CASE_INSENSITIVE.
impl Config2 {
    pub fn new(args: &[String]) -> Result<Config2, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config2 {
            query,
            filename,
            case_sensitive,
        })
    }
}
