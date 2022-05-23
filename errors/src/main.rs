#![allow(dead_code)]

use std::io::ErrorKind;
use std::fs::{self, File};
use std::io::{self, Read};

fn main() {
    // Recoverable Errors with Result
    // Most errors aren’t serious enough to require the program to stop entirely.
    // Sometimes, when a function fails, it’s for a reason that you can easily
    // interpret and respond to.

    // This return type means the call to File::open might succeed and return
    // a file handle that we can read from or write to. The function call also
    // might fail: for example, the file might not exist, or we might not have
    // permission to access the file. The File::open function needs to have
    // a way to tell us whether it succeeded or failed and at the same time
    // give us either the file handle or error information. This information
    // is exactly what the Result enum conveys.

    // In the case where File::open succeeds, the value in the variable f will
    // be an instance of Ok that contains a file handle. In the case where
    // it fails, the value in f will be an instance of Err that contains more
    // information about the kind of error that happened.
    let f = File::open("hello.txt");

    // When the result is Ok, this code will return the inner file value out of
    // the Ok variant, and we then assign that file handle value to the variable
    // f. After the match, we can use the file handle for reading or writing.
    // The other arm of the match handles the case where we get an Err value
    // from File::open. In this example, we’ve chosen to call the panic! macro.
    let _f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    // Matching on Different Errors
    // However, we want to take different actions for different failure reasons:
    // if File::open failed because the file doesn’t exist, we want to create
    // the file and return the handle to the new file. If File::open failed for
    // any other reason—for example, because we didn’t have permission to open
    // the file—we still want the code to panic!
    let f = File::open("hello.txt");
    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    // Alternatives to Using match with Result<T, E>
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Shortcuts for Panic on Error: unwrap and expect
    // Using match works well enough, but it can be a bit verbose and doesn’t
    // always communicate intent well. The Result<T, E> type has many helper
    // methods defined on it to do various, more specific tasks. The unwrap
    // method is a shortcut method implemented just like the match expression.
    let _f = File::open("hello.txt").unwrap();
    
    // Similarly, the expect method lets us also choose the panic! error message.
    // Using expect instead of unwrap and providing good error messages can
    // convey your intent and make tracking down the source of a panic easier.
    let _f = File::open("hello.txt").expect("Failed to open hello.txt");

    // Where The ? Operator Can Be Used
    // The ? operator can only be used in functions whose return type is compatible
    // with the value the ? is used on. This is because the ? operator is defined
    // to perform an early return of a value out of the function.
}

// Propagating Errors
// When a function’s implementation calls something that might fail,
// instead of handling the error within the function itself, you can
// return the error to the calling code so that it can decide what to do.
// This is known as propagating the error and gives more control to the
// calling code, where there might be more information or logic that
// dictates how the error should be handled than what you have available
// in the context of your code.

// Result<String, io::Error>. This means the function is returning a value
// of the type Result<T, E> where the generic parameter T has been filled
// in with the concrete type String, and the generic type E has been filled
// in with the concrete type io::Error. If this function succeeds without
// any problems, the code that calls this function will receive an Ok value
// that holds a String—the username that this function read from the file.

// If this function encounters any problems, the calling code will receive an
// Err value that holds an instance of io::Error that contains more information
// about what the problems were. We chose io::Error as the return type of this
// function because that happens to be the type of the error value returned from
// both of the operations we’re calling in this function’s body that might fail:
// the File::open function and the read_to_string method.
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// The code that calls this code will then handle getting either an Ok value that
// contains a username or an Err value that contains an io::Error. It’s up to the
// calling code to decide what to do with those values. If the calling code gets
// an Err value, it could call panic! and crash the program, use a default username,
// or look up the username from somewhere other than a file, for example. We don’t 
// have enough information on what the calling code is actually trying to do, so
// we propagate all the success or error information upward for it to handle appropriately.
// This pattern of propagating errors is so common in Rust that Rust provides
// the question mark operator ? to make this easier.
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// The ? operator eliminates a lot of boilerplate and makes this function’s
// implementation simpler. We could even shorten this code further by chaining
// method calls immediately after the ?.
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// Reading a file into a string is a fairly common operation, so the standard
// library provides the convenient fs::read_to_string function that opens the file,
// creates a new String, reads the contents of the file, puts the contents into
// that String, and returns it.
fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

