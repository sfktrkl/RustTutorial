fn main() {
    // By default variables are immutable
    // When a variable is immutable, once a value is bound to a name,
    // you can’t change that value.

    // You can make them mutable by adding mut in front of the variable name.
    // Adding mut also conveys intent to future readers of the code by indicating
    // that other parts of the code will be changing this variable’s value.
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Like immutable variables, constants are values that are bound to a name
    // and are not allowed to change, but there are a few differences between
    // constants and variables.
    // First, you aren’t allowed to use mut with constants. Constants aren’t
    // just immutable by default—they’re always immutable.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    // Rustaceans say that the first variable is shadowed by the second,
    // which means that the second variable’s value is what the program sees
    // when the variable is used. We can shadow a variable by using the same
    // variable’s name and repeating the use of the let keyword
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);

    // The other difference between mut and shadowing is that because
    // we’re effectively creating a new variable when we use the let keyword again,
    // we can change the type of the value but reuse the same name.
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Spaces len: {}", spaces);
}
