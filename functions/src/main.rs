fn main() {
    println!("Hello, world!");
    // Rust code uses snake case as the conventional style for function and
    // variable names, in which all letters are lowercase and underscores separate words.
    another_function();
    // We can define functions to have parameters, which are special variables
    // that are part of a function’s signature.
    print_labeled_measurement(5, 'h');

    // Function definitions are also statements.
    // Statements do not return values.
    // Therefore, you can’t assign a let statement to another variable.
    // Expressions evaluate to a value and make up most of the rest of
    // the code that you’ll write in Rust.
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

fn another_function() {
    println!("Another function.");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

// Functions can return values to the code that calls them.
// We don’t name return values, but we must declare their type after an arrow (->).
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    // Running this code will print The value of x is: 6.
    // But if we place a semicolon at the end of the line containing x + 1,
    // changing it from an expression to a statement, we’ll get an error.
    x + 1
}
