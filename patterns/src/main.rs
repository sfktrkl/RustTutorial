fn main() {
    // match Arms
    let number = 13;
    println!("Tell me about {}", number);
    // Formally, match expressions are defined as the keyword match, a value to
    // match on, and one or more match arms that consist of a pattern and an
    // expression to run if the value matches that arm’s pattern, like this:
    match number {
        // One requirement for match expressions is that they need to be
        // exhaustive in the sense that all possibilities for the value in the
        // match expression must be accounted for. One way to ensure you’ve
        // covered every possibility is to have a catchall pattern for the last
        // arm: for example, a variable name matching any value can never fail
        // and thus covers every remaining case.
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        // A particular pattern _ will match anything, but it never binds to a
        // variable, so it’s often used in the last match arm. The _ pattern can
        // be useful when you want to ignore any value not specified.
        _ => println!("Ain't special"),
    }

    // Conditional if let Expressions
    // We discussed how to use if let expressions mainly as a shorter way to
    // write the equivalent of a match that only matches one case. Optionally,
    // if let can have a corresponding else containing code to run if the
    // pattern in the if let doesn’t match.

    // Doing so gives us more flexibility than a match expression in which we
    // can express only one value to compare with the patterns. Also, the
    // conditions in a series of if let, else if, else if let arms aren’t
    // required to relate to each other.
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    // If the user specifies a favorite color, that color is the background
    // color. If today is Tuesday, the background color is green. If the user
    // specifies their age as a string and we can parse it as a number
    // successfully, the color is either purple or orange depending on the value
    // of the number. If none of these conditions apply, the background color is
    // blue.
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    // You can see that if let can also introduce shadowed variables in the same
    // way that match arms can: the line if let Ok(age) = age introduces a new
    // shadowed age variable that contains the value inside the Ok variant. This
    // means we need to place the if age > 30 condition within that block: we
    // can’t combine these two conditions into if let Ok(age) = age && age > 30.
    // The shadowed age we want to compare to 30 isn’t valid until the new scope
    // starts with the curly bracket.
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    // The downside of using if let expressions is that the compiler doesn’t
    // check exhaustiveness, whereas with match expressions it does. If we
    // omitted the last else block and therefore missed handling some cases, the
    // compiler would not alert us to the possible logic bug.
    } else {
        println!("Using blue as the background color");
    }

    // while let Conditional Loops
    // Similar in construction to if let, the while let conditional loop allows
    // a while loop to run for as long as a pattern continues to match.
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // This example prints 3, 2, and then 1. The pop method takes the last
    // element out of the vector and returns Some(value). If the vector is
    // empty, pop returns None. The while loop continues running the code in its
    // block as long as pop returns Some. When pop returns None, the loop stops.
    // We can use while let to pop every element off our stack.
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for Loops
    // In a for loop, the pattern is the value that directly follows the keyword
    // for, so in for x in y the x is the pattern.
    let v = vec!['a', 'b', 'c'];

    // We use the enumerate method to adapt an iterator to produce a value and
    // that value’s index in the iterator, placed into a tuple. The first value
    // produced is the tuple (0, 'a'). When this value is matched to the pattern
    // (index, value), index will be 0 and value will be 'a', printing the first
    // line of the output.
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // let Statements
    // We had only explicitly discussed using patterns with match and if let,
    // but in fact, we’ve used patterns in other places as well, including in
    // let statements. For example, consider this straightforward variable
    // assignment with let:
    let _x = 5;

    // In statements like let x = 5; with a variable name in the PATTERN slot,
    // the variable name is just a particularly simple form of a pattern. Rust
    // compares the expression against the pattern and assigns any names it
    // finds. So in the let x = 5; example, x is a pattern that means “bind what
    // matches here to the variable x.” Because the name x is the whole pattern,
    // this pattern effectively means “bind everything to the variable x,
    // whatever the value is.”

    // Here, we match a tuple against a pattern. Rust compares the value (1, 2,
    // 3) to the pattern (x, y, z) and sees that the value matches the pattern,
    // so Rust binds 1 to x, 2 to y, and 3 to z. You can think of this tuple
    // pattern as nesting three individual variable patterns inside it.
    let (_x, _y, _z) = (1, 2, 3);

    // Function Parameters
    // Function parameters can also be patterns.
    let point = (3, 5);
    print_coordinates(&point);
}

// This code prints Current location: (3, 5). The values &(3, 5) match the
// pattern &(x, y), so x is the value 3 and y is the value 5.

// We can also use patterns in closure parameter lists in the same way as in
// function parameter lists, because closures are similar to functions.
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
