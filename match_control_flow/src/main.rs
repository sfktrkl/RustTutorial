#![allow(dead_code)]
// The match Control Flow Construct
// Rust has an extremely powerful control flow construct called match that
// allows you to compare a value against a series of patterns and then execute
// code based on which pattern matches. Patterns can be made up of literal values,
// variable names, wildcards, and many other things.
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

// Patterns that Bind to Values
// Another useful feature of match arms is that they can bind to the parts of
// the values that match the pattern. This is how we can extract values out of
// enum variants.
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let coin = Coin::Penny;
    value_in_cents(coin);

    let coin2 = Coin2::Quarter(UsState::Alaska);
    value_in_cents2(coin2);

    let _five = Some(5);
    let _six = plus_one(_five);
    let _none = plus_one(None);

    // Catch-all Patterns and the _ Placeholder
    // This code compiles, even though we haven’t listed all the possible values
    // a u8 can have, because the last pattern will match all values
    // not specifically listed. This catch-all pattern meets the requirement
    // that match must be exhaustive.
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    // This example also meets the exhaustiveness requirement because
    // we’re explicitly ignoring all other values in the last arm;
    // we haven’t forgotten anything.
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_in_cents2(coin: Coin2) -> u8 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickel => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// Matching with Option<T>
// In the previous section, we wanted to get the inner T value out of the Some
// case when using Option<T>; we can also handle Option<T> using match as we did
// with the Coin enum! Instead of comparing coins, we’ll compare the variants of
// Option<T>, but the way that the match expression works remains the same.
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        // If we didn’t handle the None case, so this code will cause a bug.
        // Luckily, it’s a bug Rust knows how to catch.
        // Rust knows that we didn’t cover every possible case and even knows
        // which pattern we forgot! Matches in Rust are exhaustive:
        // we must exhaust every last possibility in order for the code to be valid.
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn reroll() {}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(_num_spaces: u8) {}
