#![allow(dead_code)]

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // Concise Control Flow with if let
    // The if let syntax lets you combine if and let into a less verbose way to
    // handle values that match one pattern while ignoring the rest.
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // If the value is Some, we print out the value in the Some variant
    // by binding the value to the variable max in the pattern.
    // We don’t want to do anything with the None value.
    // To satisfy the match expression, we have to add _ => ()
    // after processing just one variant, which is annoying boilerplate code to add.
    // Instead, we could write this in a shorter way using if let.
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // We can include an else with an if let.
    // The block of code that goes with the else is the same as the block
    // of code that would go with the _ case in the match expression
    // that is equivalent to the if let and else.
    let coin = Coin::Penny;
    let mut _count = 0;
    match coin {
        Coin::Quarter(ref state) => println!("State quarter from {:?}!", state),
        _ => _count += 1,
    }

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        _count += 1;
    }
}
