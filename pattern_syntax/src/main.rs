#![allow(dead_code)]

fn main() {
    // Matching Literals
    // You can match patterns against literals directly.
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching Named Variables
    // Named variables are irrefutable patterns that match any value, and we’ve
    // used them many times in the book. However, there is a complication when
    // you use named variables in match expressions. Because match starts a new
    // scope, variables declared as part of a pattern inside the match
    // expression will shadow those with the same name outside the match
    // construct, as is the case with all variables.
    let x = Some(5);
    let y = 10;
    match x {
        // Let’s walk through what happens when the match expression runs. The
        // pattern in the first match arm doesn’t match the defined value of x,
        // so the code continues.
        Some(50) => println!("Got 50"),
        // The pattern in the second match arm introduces a new variable named y
        // that will match any value inside a Some value. Because we’re in a new
        // scope inside the match expression, this is a new y variable, not the
        // y we declared at the beginning with the value 10. This new y binding
        // will match any value inside a Some, which is what we have in x.
        // Therefore, this new y binds to the inner value of the Some in x. That
        // value is 5, so the expression for that arm executes and prints
        // Matched, y = 5.
        Some(y) => println!("Matched, y = {:?}", y),
        // If x had been a None value instead of Some(5), the patterns in the
        // first two arms wouldn’t have matched, so the value would have matched
        // to the underscore. We didn’t introduce the x variable in the pattern
        // of the underscore arm, so the x in the expression is still the outer
        // x that hasn’t been shadowed. In this hypothetical case, the match
        // would print Default case, x = None.
        _ => println!("Default case, x = {:?}", x),
    }
    // When the match expression is done, its scope ends, and so does the scope
    // of the inner y. The last println! produces at the end: x = Some(5), y =
    // 10.
    println!("at the end: x = {:?}, y = {:?}", x, y);

    // Multiple Patterns
    // In match expressions, you can match multiple patterns using the | syntax,
    // which means or. For example, the following code matches the value of x
    // against the match arms, the first of which has an or option, meaning if
    // the value of x matches either of the values in that arm, that arm’s code
    // will run:
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Matching Ranges of Values with ..=
    // The ..= syntax allows us to match to an inclusive range of values. In the
    // following code, when a pattern matches any of the values within the
    // range, that arm will execute:
    let x = 5;
    match x {
        // If x is 1, 2, 3, 4, or 5, the first arm will match. This syntax is
        // more convenient than using the | operator to express the same idea;
        // instead of 1..=5, we would have to specify 1 | 2 | 3 | 4 | 5 if we
        // used |.
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    // Ranges are only allowed with numeric values or char values, because the
    // compiler checks that the range isn’t empty at compile time. The only
    // types for which Rust can tell if a range is empty or not are char and
    // numeric values.
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // Destructuring to Break Apart Values
    // We can also use patterns to destructure structs, enums, and tuples to use
    // different parts of these values. Let’s walk through each value.
    let p = Point { x: 0, y: 7 };

    // This code creates the variables a and b that match the values of the x
    // and y fields of the p struct. This example shows that the names of the
    // variables in the pattern don’t have to match the field names of the
    // struct. But it’s common to want the variable names to match the field
    // names to make it easier to remember which variables came from which
    // fields.
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // Because having variable names match the fields is common and because
    // writing let Point { x: x, y: y } = p; contains a lot of duplication,
    // there is a shorthand for patterns that match struct fields: you only need
    // to list the name of the struct field, and the variables created from the
    // pattern will have the same names.
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // We can also destructure with literal values as part of the struct pattern
    // rather than creating variables for all the fields. Doing so allows us to
    // test some of the fields for particular values while creating variables to
    // destructure the other fields.
    match p {
        // The first arm will match any point that lies on the x axis by
        // specifying that the y field matches if its value matches the literal
        // 0. The pattern still creates an x variable that we can use in the
        // code for this arm.
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // Similarly, the second arm matches any point on the y axis by
        // specifying that the x field matches if its value is 0 and creates a
        // variable y for the value of the y field. The third arm doesn’t
        // specify any literals, so it matches any other Point and creates
        // variables for both the x and y fields.
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // Destructuring Enums
    // One detail we haven’t mentioned explicitly is that the pattern to
    // destructure an enum should correspond to the way the data stored within
    // the enum is defined.

    // This code will print Change the color to red 0, green 160, and blue 255.
    // Try changing the value of msg to see the code from the other arms run.
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        // For enum variants without any data, like Message::Quit, we can’t
        // destructure the value any further. We can only match on the literal
        // Message::Quit value, and no variables are in that pattern.
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        // For struct-like enum variants, such as Message::Move, we can use a
        // pattern similar to the pattern we specify to match structs. After the
        // variant name, we place curly brackets and then list the fields with
        // variables so we break apart the pieces to use in the code for this
        // arm.
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        // For tuple-like enum variants, like Message::Write that holds a tuple
        // with one element and Message::ChangeColor that holds a tuple with
        // three elements, the pattern is similar to the pattern we specify to
        // match tuples. The number of variables in the pattern must match the
        // number of elements in the variant we’re matching.
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }

    // Destructuring Nested Structs and Enums
    // Until now, all our examples have been matching structs or enums that were
    // one level deep. Matching can work on nested items too!
    let msg = Message2::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        // The pattern of the first arm in the match expression matches a
        // Message::ChangeColor enum variant that contains a Color::Rgb variant;
        // then the pattern binds to the three inner i32 values. The pattern of
        // the second arm also matches a Message::ChangeColor enum variant, but
        // the inner enum matches the Color::Hsv variant instead. We can specify
        // these complex conditions in one match expression, even though two
        // enums are involved.
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }

    // Destructuring Structs and Tuples
    // We can mix, match, and nest destructuring patterns in even more complex
    // ways. The following example shows a complicated destructure where we nest
    // structs and tuples inside a tuple and destructure all the primitive
    // values out:
    let ((_feet, _inches), Point { x: _, y: _ }) = ((3, 10), Point { x: 3, y: -10 });

    // Ignoring Values in a Pattern
    // You’ve seen that it’s sometimes useful to ignore values in a pattern,
    // such as in the last arm of a match, to get a catchall that doesn’t
    // actually do anything but does account for all remaining possible values.
    // There are a few ways to ignore entire values or parts of values in a
    // pattern: using the _ pattern (which you’ve seen), using the _ pattern
    // within another pattern, using a name that starts with an underscore, or
    // using .. to ignore remaining parts of a value. Let’s explore how and why
    // to use each of these patterns.

    // Ignoring an Entire Value with _
    // We’ve used the underscore (_) as a wildcard pattern that will match any
    // value but not bind to the value. Although the underscore _ pattern is
    // especially useful as the last arm in a match expression, we can use it in
    // any pattern.

    // Ignoring Parts of a Value with a Nested _
    // We can also use _ inside another pattern to ignore just part of a value,
    // for example, when we want to test for only part of a value but have no
    // use for the other parts in the corresponding code we want to run. Listing
    // 18-18 shows code responsible for managing a setting’s value. The business
    // requirements are that the user should not be allowed to overwrite an
    // existing customization of a setting but can unset the setting and give it
    // a value if it is currently unset.
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    // This code will print Can't overwrite an existing customized value and
    // then setting is Some(5). In the first match arm, we don’t need to match
    // on or use the values inside either Some variant, but we do need to test
    // for the case when setting_value and new_setting_value are the Some
    // variant. In that case, we print why we’re not changing setting_value, and
    // it doesn’t get changed.
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        // In all other cases (if either setting_value or new_setting_value are
        // None) expressed by the _ pattern in the second arm, we want to allow
        // new_setting_value to become setting_value.
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    // We can also use underscores in multiple places within one pattern to
    // ignore particular values.
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }

    // Ignoring an Unused Variable by Starting Its Name with _
    // If you create a variable but don’t use it anywhere, Rust will usually
    // issue a warning because that could be a bug. But sometimes it’s useful to
    // create a variable you won’t use yet, such as when you’re prototyping or
    // just starting a project. In this situation, you can tell Rust not to warn
    // you about the unused variable by starting the name of the variable with
    // an underscore. In Listing 18-20, we create two unused variables, but when
    // we compile this code, we should only get a warning about one of them.
    let _x = 5;

    // Note that there is a subtle difference between using only _ and using a
    // name that starts with an underscore. The syntax _x still binds the value
    // to the variable, whereas _ doesn’t bind at all.
    let s = Some(String::from("Hello!"));
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    // Ignoring Remaining Parts of a Value with ..
    // With values that have many parts, we can use the .. syntax to use only a
    // few parts and ignore the rest, avoiding the need to list underscores for
    // each ignored value. The .. pattern ignores any parts of a value that we
    // haven’t explicitly matched in the rest of the pattern.
    let origin = Point2 { x: 0, y: 0, z: 0 };
    match origin {
        // We list the x value and then just include the .. pattern. This is
        // quicker than having to list y: _ and z: _, particularly when we’re
        // working with structs that have lots of fields in situations where
        // only one or two fields are relevant.
        Point2 { x, .. } => println!("x is {}", x),
    }

    // The syntax .. will expand to as many values as it needs to be.
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    // Extra Conditionals with Match Guards
    // A match guard is an additional if condition specified after the pattern
    // in a match arm that must also match, along with the pattern matching, for
    // that arm to be chosen. Match guards are useful for expressing more
    // complex ideas than a pattern alone allows.
    let num = Some(4);
    match num {
        // This example will print The number 4 is even. When num is compared to
        // the pattern in the first arm, it matches, because Some(4) matches
        // Some(x). Then the match guard checks whether the remainder of
        // dividing x by 2 is equal to 0, and because it is, the first arm is
        // selected.
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        // If num had been Some(5) instead, the match guard in the first arm
        // would have been false because the remainder of 5 divided by 2 is 1,
        // which is not equal to 0. Rust would then go to the second arm, which
        // would match because the second arm doesn’t have a match guard and
        // therefore matches any Some variant.
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    // We mentioned that we could use match guards to solve our
    // pattern-shadowing problem. Recall that a new variable was created inside
    // the pattern in the match expression instead of using the variable outside
    // the match. That new variable meant we couldn’t test against the value of
    // the outer variable.
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        // This code will now print Default case, x = Some(5). The pattern in
        // the second match arm doesn’t introduce a new variable y that would
        // shadow the outer y, meaning we can use the outer y in the match
        // guard. Instead of specifying the pattern as Some(y), which would have
        // shadowed the outer y, we specify Some(n). This creates a new variable
        // n that doesn’t shadow anything because there is no n variable outside
        // the match.
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {}", x, y);

    // You can also use the or operator | in a match guard to specify multiple
    // patterns; the match guard condition will apply to all the patterns.
    // Listing 18-28 shows the precedence of combining a match guard with a
    // pattern that uses |. The important part of this example is that the if y
    // match guard applies to 4, 5, and 6, even though it might look like if y
    // only applies to 6.
    let x = 4;
    let y = false;
    match x {
        // The match condition states that the arm only matches if the value of
        // x is equal to 4, 5, or 6 and if y is true. When this code runs, the
        // pattern of the first arm matches because x is 4, but the match guard
        // if y is false, so the first arm is not chosen. The code moves on to
        // the second arm, which does match, and this program prints no. The
        // reason is that the if condition applies to the whole pattern 4 | 5 |
        // 6, not only to the last value 6.
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @ Bindings
    // The at operator (@) lets us create a variable that holds a value at the
    // same time we’re testing that value to see whether it matches a pattern.
    let msg = Message3::Hello { id: 5 };
    match msg {
        // This example will print Found an id in range: 5. By specifying
        // id_variable @ before the range 3..=7, we’re capturing whatever value
        // matched the range while also testing that the value matched the range
        // pattern.
        Message3::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        // In the second arm, where we only have a range specified in the
        // pattern, the code associated with the arm doesn’t have a variable
        // that contains the actual value of the id field. The id field’s value
        // could have been 10, 11, or 12, but the code that goes with that
        // pattern doesn’t know which it is. The pattern code isn’t able to use
        // the value from the id field, because we haven’t saved the id value in
        // a variable.
        Message3::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        // In the last arm, where we’ve specified a variable without a range, we
        // do have the value available to use in the arm’s code in a variable
        // named id. The reason is that we’ve used the struct field shorthand
        // syntax. But we haven’t applied any test to the value in the id field
        // in this arm, as we did with the first two arms: any value would match
        // this pattern.
        Message3::Hello { id } => println!("Found some other id: {}", id),
    }
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Message2 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

// This code will completely ignore the value passed as the first argument, 3,
// and will print This code only uses the y parameter: 4.

// In most cases when you no longer need a particular function parameter, you
// would change the signature so it doesn’t include the unused parameter.
// Ignoring a function parameter can be especially useful in some cases, for
// example, when implementing a trait when you need a certain type signature but
// the function body in your implementation doesn’t need one of the parameters.
// The compiler will then not warn about unused function parameters, as it would
// if you used a name instead.
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

struct Point2 {
    x: i32,
    y: i32,
    z: i32,
}

enum Message3 {
    Hello { id: i32 },
}
