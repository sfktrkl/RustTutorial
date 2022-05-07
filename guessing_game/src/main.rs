use rand::Rng;
use std::cmp::Ordering;
// input/output library
use std::io;

// the main function is the entry point into the program
fn main() {
    // println! is a macro that prints a string to the screen
    println!("Guess the number!");

    // we call the rand::thread_rng function that gives us the particular random
    // number generator that we’re going to use: one that is local to the current
    //thread of execution and seeded by the operating system.
    let secret_number = rand::thread_rng().gen_range(1..101);

    // The loop keyword creates an infinite loop.
    loop {
        println!("Please input your guess.");

        // In Rust, variables are immutable by default.
        // To make a variable mutable, we add mut before the variable name.
        let mut guess = String::new();

        // The stdin function returns an instance of std::io::Stdin.
        // Next, the line .read_line(&mut guess) calls the read_line method on
        // the standard input handle to get input from the user.
        // We’re also passing &mut guess as the argument to read_line to tell
        // it what string to store the user input in.
        // The & indicates that this argument is a reference.
        // Like variables, references are immutable by default.

        // read_line puts whatever the user enters into the string we pass to it,
        // but it also returns a value—in this case, an io::Result.
        // Result’s variants are Ok and Err.
        // Values of the Result type, like values of any type, have methods defined
        // on them. An instance of io::Result has an expect method that you can call.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // The {} set of curly brackets is a placeholder.
        println!("You guessed: {}", guess);

        // we want to convert the String the program reads as input into a real
        // number type so we can compare it numerically to the secret number. We do
        //so by adding this line to the main function body.
        // We create a variable named guess. But wait, doesn’t the program already
        // have a variable named guess? It does, but helpfully Rust allows us to shadow
        // the previous value of guess with a new one. Shadowing lets us reuse the guess
        // variable name rather than forcing us to create two unique variables, such as
        // guess_str and guess for example.

        // To further refine the game’s behavior, rather than crashing the program
        // when the user inputs a non-number, let’s make the game ignore
        // a non-number so the user can continue guessing.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // The Ordering type is another enum and has the variants Less, Greater, and
        // Equal. These are the three outcomes that are possible when you compare two values.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
