fn main() {
    // Generic Types, Traits, and Lifetimes
    // Every programming language has tools for effectively handling the
    // duplication of concepts. In Rust, one such tool is generics:
    // abstract stand-ins for concrete types or other properties.
    // We can express the behavior of generics or how they relate to other
    // generics without knowing what will be in their place when compiling
    // and running the code.

    // Removing Duplication by Extracting a Function
    // Generics allow us to replace specific types with a placeholder that represents
    // multiple types to remove code duplication. Before diving into generics syntax,
    // then, let’s first look at how to remove duplication in a way that doesn’t
    // involve generic types by extracting a function that replaces specific
    // values with a placeholder that represents multiple values.
    // We've now been tasked with finding the largest number in two
    // different lists of numbers. To do so, we can choose to duplicate the code.
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    // To eliminate this duplication, we’ll create an abstraction by defining
    // a function that operates on any list of integers passed in a parameter.
    // This solution makes our code clearer and lets us express the concept of
    // finding the largest number in a list abstractly.
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest_value(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest_value(&number_list);
    println!("The largest number is {}", result);
}

// The largest function has a parameter called list, which represents any
// concrete slice of i32 values we might pass into the function. As a result,
// when we call the function, the code runs on the specific values that we pass in.
fn largest_value(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
