fn main() {
    // Unlike languages such as Ruby and JavaScript, Rust will not automatically
    // try to convert non-Boolean types to a Boolean. You must be explicit and
    // always provide if with a Boolean as its condition.
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // You can use multiple conditions by combining if and else in an else if expression.
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Because if is an expression, we can use it on the right side of
    // a let statement to assign the outcome to a variable.
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // The loop keyword tells Rust to execute a block of code over and over
    // again forever or until you explicitly tell it to stop.
    // If you have loops within loops, break and continue apply to the innermost
    // loop at that point. You can optionally specify a loop label on a loop
    // that we can then use with break or continue to specify that those keywords
    // apply to the labeled loop instead of the innermost loop.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    // One of the uses of a loop is to retry an operation you know might fail,
    // such as checking whether a thread has completed its job. You might also
    // need to pass the result of that operation out of the loop to the rest
    // of your code. To do this, you can add the value you want returned after
    // the break expression you use to stop the loop; that value will be
    // returned out of the loop so you can use it.
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // A program will often need to evaluate a condition within a loop.
    // While the condition is true, the loop runs. When the condition ceases
    // to be true, the program calls break, stopping the loop. It’s possible
    // to implement behavior like this using a combination of loop, if, else,
    // and break; you could try that now in a program, if you’d like. However,
    // this pattern is so common that Rust has a built-in language construct
    // for it, called a while loop.
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // You can choose to use the while construct to loop over the elements
    // of a collection, such as an array.
    // However, this approach is error prone; we could cause the program to
    // panic if the index value or test condition are incorrect. For example,
    // if you changed the definition of the a array to have four elements but
    // forgot to update the condition to while index < 4, the code would panic.
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // As a more concise alternative, you can use a for loop and execute
    // some code for each item in a collection.
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }

    // Here’s what the countdown would look like using a for loop and
    // another method we’ve not yet talked about, rev, to reverse the range.
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
