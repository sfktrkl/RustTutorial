use minigrep::Config;
use minigrep::Config2;
use minigrep::Config3;
use std::env;
use std::fs;
use std::process;

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

    // Refactoring to Improve Modularity and Error Handling
    // First, our main function now performs two tasks: it parses arguments and
    // reads files. For such a small function, this isn’t a major problem. However,
    // if we continue to grow our program inside main, the number of separate
    // tasks the main function handles will increase. As a function gains
    // responsibilities, it becomes more difficult to reason about, harder
    // to test, and harder to change without breaking one of its parts. It’s
    // best to separate functionality so each function is responsible for one task.

    // This issue also ties into the second problem: although query and filename
    // are configuration variables to our program, variables like contents are
    // used to perform the program’s logic. The longer main becomes, the more
    // variables we’ll need to bring into scope; the more variables we have in
    // scope, the harder it will be to keep track of the purpose of each. It’s
    // best to group the configuration variables into one structure to make their
    // purpose clear.

    // The third problem is that we’ve used expect to print an error message when
    // reading the file fails, but the error message just prints Something went
    // wrong reading the file. Reading a file can fail in a number of ways: for
    // example, the file could be missing, or we might not have permission to open it.

    // Fourth, we use expect repeatedly to handle different errors, and if the
    // user runs our program without specifying enough arguments, they’ll get
    // an index out of bounds error from Rust that doesn’t clearly explain the
    // problem. It would be best if all the error-handling code were in one place
    // so future maintainers had only one place to consult in the code if the error
    //-handling logic needed to change. Having all the error-handling code in
    // one place will also ensure that we’re printing messages that will be
    // meaningful to our end users.

    // Separation of Concerns for Binary Projects
    // The organizational problem of allocating responsibility for multiple
    // tasks to the main function is common to many binary projects.
    //  Split your program into a main.rs and a lib.rs and move your program’s
    //      logic to lib.rs.
    //  As long as your command line parsing logic is small, it can remain in main.rs.
    //  When the command line parsing logic starts getting complicated, extract
    //      it from main.rs and move it to lib.rs.

    // The responsibilities that remain in the main function after this process
    // should be limited to the following:
    //  Calling the command line parsing logic with the argument values
    //  Setting up any other configuration
    //  Calling a run function in lib.rs
    //  Handling the error if run returns an error

    // Extracting the Argument Parser
    // We’ll extract the functionality for parsing arguments into a function
    // that main will call to prepare for moving the command line parsing logic
    // to src/lib.rs.
    let args: Vec<String> = env::args().collect();
    let (_query, _filename) = minigrep::parse_config(&args);

    // Grouping Configuration Values
    // We can take another small step to improve the parse_config function further.
    // At the moment, we’re returning a tuple, but then we immediately break that
    // tuple into individual parts again. This is a sign that perhaps we don’t
    // have the right abstraction yet.
    // Another indicator that shows there’s room for improvement is the config
    // part of parse_config, which implies that the two values we return are
    // related and are both part of one configuration value. We’re not currently
    // conveying this meaning in the structure of the data other than by grouping
    // the two values into a tuple; we could put the two values into one struct
    // and give each of the struct fields a meaningful name.
    let args: Vec<String> = env::args().collect();
    let config = minigrep::parse_config2(&args);
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // So now that the purpose of the parse_config function is to create a Config
    // instance, we can change parse_config from a plain function to a function
    // named new that is associated with the Config struct. Making this change
    // will make the code more idiomatic. We can create instances of types in the
    // standard library, such as String, by calling String::new.
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // Calling Config::new and Handling Errors
    // In this listing, we’ve used a method we haven’t covered in detail yet:
    // unwrap_or_else, which is defined on Result<T, E> by the standard library.
    // Using unwrap_or_else allows us to define some custom, non-panic! error
    // handling. If the Result is an Ok value, this method’s behavior is similar
    // to unwrap: it returns the inner value Ok is wrapping. However, if the value
    // is an Err value, this method calls the code in the closure, which is an
    // anonymous function we define and pass as an argument to unwrap_or_else.
    let args: Vec<String> = env::args().collect();
    let config = Config::new2(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let args: Vec<String> = env::args().collect();
    let config = Config::new2(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    minigrep::run(config);

    let args: Vec<String> = env::args().collect();
    let config = Config::new2(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // Handling Errors Returned from run in main
    // We’ll check for errors and handle them using a technique similar to one
    // we used with Config::new.
    if let Err(e) = minigrep::run2(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }

    // Using the search Function in the run Function
    let args: Vec<String> = env::args().collect();
    let config = Config::new2(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run3(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
    //$ cargo run frog poem.txt
    //$ cargo run body poem.txt
    //$ cargo run monomorphization poem.txt

    let args: Vec<String> = env::args().collect();
    let config = Config2::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run4(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
    //PS> $Env:CASE_INSENSITIVE=1; cargo run to poem.txt
    //PS> Remove-Item Env:CASE_INSENSITIVE

    // Writing Error Messages to Standard Error Instead of Standard Output
    // At the moment, we’re writing all of our output to the terminal using the
    // println! macro. In most terminals, there are two kinds of output: standard
    // output (stdout) for general information and standard error (stderr) for
    // error messages. This distinction enables users to choose to direct the
    // successful output of a program to a file but still print error messages
    // to the screen.
    //The println! macro is only capable of printing to standard output, so we
    // have to use something else to print to standard error.

    // Checking Where Errors Are Written
    // First, let’s observe how the content printed by minigrep is currently
    // being written to standard output, including any error messages we want
    // to write to standard error instead. We’ll do that by redirecting the
    // standard output stream to a file while also intentionally causing an error.
    // We won’t redirect the standard error stream, so any content sent to
    // standard error will continue to display on the screen.
    // Command line programs are expected to send error messages to the standard
    // error stream so we can still see error messages on the screen even if we
    // redirect the standard output stream to a file. Our program is not currently
    // well-behaved: we’re about to see that it saves the error message output to
    // a file instead!
    //$ cargo run > output.txt

    // Printing Errors to Standard Error
    // The standard library provides the eprintln! macro that prints to the
    // standard error stream, so let’s change the two places we were calling
    // println! to print errors to use eprintln! instead.
    let args: Vec<String> = env::args().collect();
    let config = Config::new2(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run3(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    //$ cargo run > output.txt
    //$ cargo run to poem.txt > output.txt

    // Using the Returned Iterator Directly
    // The env::args function returns an iterator! Rather than collecting the
    // iterator values into a vector and then passing a slice to Config::new,
    // now we’re passing ownership of the iterator returned from env::args to
    // Config::new directly.
    let config = Config3::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run5(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
