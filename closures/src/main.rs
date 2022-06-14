use std::thread;
use std::time::Duration;

fn main() {
    // Rust’s closures are anonymous functions you can save in a variable or
    // pass as arguments to other functions. You can create the closure in
    // one place and then call the closure to evaluate it in a different context.
    // Unlike functions, closures can capture values from the scope in which
    // they’re defined.

    // Let’s work on an example of a situation in which it’s useful to store
    // a closure to be executed later.
    // We’ll simulate calling this hypothetical algorithm with the function.
    // Next is the main function, which contains the parts of the workout app
    // important for this example. This function represents the code that the
    // app will call when a user asks for a workout plan. Because the interaction
    // with the app’s frontend isn’t relevant to the use of closures, we’ll
    // hardcode values representing inputs to our program and print the outputs.
    // The required inputs are these:
    //  An intensity number from the user, which is specified when they request
    //      a workout to indicate whether they want a low-intensity workout or
    //      a high-intensity workout
    //  A random number that will generate some variety in the workout plans
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;
    generate_workout(simulated_user_specified_value, simulated_random_number);
    generate_workout2(simulated_user_specified_value, simulated_random_number);
    generate_workout3(simulated_user_specified_value, simulated_random_number);

    // Closure Type Inference and Annotation
    // Closures don’t require you to annotate the types of the parameters or the
    // return value like fn functions do. Type annotations are required on functions
    // because they’re part of an explicit interface exposed to your users.
    // Defining this interface rigidly is important for ensuring that everyone
    // agrees on what types of values a function uses and returns. But closures
    // aren’t used in an exposed interface like this: they’re stored in variables
    // and used without naming them and exposing them to users of our library.
    // As with variables, we can add type annotations if we want to increase
    // explicitness and clarity at the cost of being more verbose than is strictly
    // necessary.
    let _expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // With type annotations added, the syntax of closures looks more similar
    // to the syntax of functions. The following is a vertical comparison of the
    // syntax for the definition of a function that adds 1 to its parameter and
    // a closure that has the same behavior. We’ve added some spaces to line up
    // the relevant parts. This illustrates how closure syntax is similar to
    // function syntax except for the use of pipes and the amount of syntax
    // that is optional:
    // The first line shows a function definition, and the second line shows
    // a fully annotated closure definition. The third line removes the type
    // annotations from the closure definition, and the fourth line removes
    // the brackets, which are optional because the closure body has only one
    // expression. These are all valid definitions that will produce the same
    // behavior when they’re called. Calling the closures is required for
    // add_one_v5 to be able to compile because the types will be inferred from
    // their usage.
    fn  _add_one_v1   (x: u32) -> u32 { x + 1 }
    let _add_one_v2 = |x: u32| -> u32 { x + 1 };
    let _add_one_v3 = |x: u32|        { x + 1 };
    let _add_one_v4 = |x: u32|          x + 1  ;
    let _add_one_v5 = |x|               x + 1  ;
    let _two = _add_one_v5(1);

    // Closure definitions will have one concrete type inferred for each of
    // their parameters and for their return value. For instance,the definition
    // of a short closure that just returns the value it receives as a parameter.
    // This closure isn’t very useful except for the purposes of this example.
    // Note that we haven’t added any type annotations to the definition: if we
    // then try to call the closure twice, using a String as an argument the
    // first time and a u32 the second time, we’ll get an error.
    let example_closure = |x| x;
    let _s = example_closure(String::from("hello"));
    //let n = example_closure(5);

    // Instead of saving the closure in a variable directly, we save a new
    // instance of Cacher that holds the closure. Then, in each place we want
    // the result, we call the value method on the Cacher instance. We can call
    // the value method as many times as we want, or not call it at all, and the
    // expensive calculation will be run a maximum of once.
    generate_workout4(simulated_user_specified_value, simulated_random_number);

    // Capturing the Environment with Closures
    // In the workout generator example, we only used closures as inline anonymous
    // functions. However, closures have an additional capability that functions
    // don’t have: they can capture their environment and access variables from
    // the scope in which they’re defined.
    let x = 4;
    // Here, even though x is not one of the parameters of equal_to_x,
    // the equal_to_x closure is allowed to use the x variable that’s defined
    // in the same scope that equal_to_x is defined in. We can’t do the same
    // with functions.
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    // Closures can capture values from their environment in three ways, which
    // directly map to the three ways a function can take a parameter: taking
    // ownership, borrowing mutably, and borrowing immutably. These are encoded
    // in the three Fn traits as follows:
    //  FnOnce consumes the variables it captures from its enclosing scope,
    //      known as the closure’s environment. To consume the captured variables,
    //      the closure must take ownership of these variables and move them into
    //      the closure when it is defined. The Once part of the name represents
    //      the fact that the closure can’t take ownership of the same variables
    //      more than once, so it can be called only once.
    //  FnMut can change the environment because it mutably borrows values.
    //  Fn borrows values from the environment immutably.
    // When you create a closure, Rust infers which trait to use based on how
    // the closure uses the values from the environment. All closures implement
    // FnOnce because they can all be called at least once. Closures that don’t
    // move the captured variables also implement FnMut, and closures that don’t
    // need mutable access to the captured variables also implement Fn.
    // f you want to force the closure to take ownership of the values it uses
    // in the environment, you can use the move keyword before the parameter list.
    // This technique is mostly useful when passing a closure to a new thread to
    // move the data so it’s owned by the new thread.
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;

    // The x value is moved into the closure when the closure is defined,
    // because we added the move keyword. The closure then has ownership of x,
    // and main isn’t allowed to use x anymore in the println! statement.
    //println!("can't use x here: {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

// Now that we have the context, let’s get to the algorithm. The function
// generate_workout contains the business logic of the app that we’re most
// concerned with in this example.
fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
    }
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

// Refactoring Using Functions
// We could restructure the workout program in many ways. First, we’ll try
// extracting the duplicated call to the simulated_expensive_calculation
// function into a variable.
// This change unifies all the calls to simulated_expensive_calculation and
// solves the problem of the first if block unnecessarily calling the function
// twice. Unfortunately, we’re now calling this function and waiting for
// the result in all cases, which includes the inner if block that doesn’t
// use the result value at all.
fn generate_workout2(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result);
        println!("Next, do {} situps!", expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result);
        }
    }
}

// Refactoring with Closures to Store Code
// Instead of always calling the simulated_expensive_calculation function before
// the if blocks, we can define a closure and store the closure in a variable
// rather than storing the result of the function call. We can actually move
// the whole body of simulated_expensive_calculation within the closure we’re
// introducing here.
fn generate_workout3(intensity: u32, random_number: u32) {
    // The closure definition comes after the = to assign it to the variable
    // expensive_closure. To define a closure, we start with a pair of vertical
    // pipes (|), inside which we specify the parameters to the closure; this
    // syntax was chosen because of its similarity to closure definitions in
    // Smalltalk and Ruby. This closure has one parameter named num: if we had
    // more than one parameter, we would separate them with commas,
    // like |param1, param2|.
    // After the parameters, we place curly brackets that hold the body of the
    // closure—these are optional if the closure body is a single expression.
    // The end of the closure, after the curly brackets, needs a semicolon to
    // complete the let statement. The value returned from the last line in the
    // closure body (num) will be the value returned from the closure when it’s
    // called, because that line doesn’t end in a semicolon; just as in function
    // bodies.
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        // However, we’ve reintroduced one of the problems, we’re still calling
        // the closure twice in the first if block, which will call the expensive
        // code twice and make the user wait twice as long as they need to. We
        // could fix this problem by creating a variable local to that if block
        // to hold the result of calling the closure, but closures provide us with
        // another solution. We’ll talk about that solution in a bit.
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}

// Storing Closures Using Generic Parameters and the Fn Traits
// We can create a struct that will hold the closure and the resulting value
// of calling the closure. The struct will execute the closure only if we need
// the resulting value, and it will cache the resulting value so the rest of
// our code doesn’t have to be responsible for saving and reusing the result.
// You may know this pattern as memoization or lazy evaluation.
// To make a struct that holds a closure, we need to specify the type of the
// closure, because a struct definition needs to know the types of each of
// its fields. Each closure instance has its own unique anonymous type:
// that is, even if two closures have the same signature, their types are
// still considered different.
// The Fn traits are provided by the standard library. All closures implement
// at least one of the traits: Fn, FnMut, or FnOnce.
// We add types to the Fn trait bound to represent the types of the parameters
// and return values the closures must have to match this trait bound.
// In this case, our closure has a parameter of type u32 and returns a u32,
// so the trait bound we specify is Fn(u32) -> u32.
struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

// The Cacher struct has a calculation field of the generic type T. The trait
// bounds on T specify that it’s a closure by using the Fn trait. Any closure
// we want to store in the calculation field must have one u32 parameter
// (specified within the parentheses after Fn) and must return a u32
// (specified after the ->).
// The value field is of type Option<u32>. Before we execute the closure, value
// will be None. When code using a Cacher asks for the result of the closure,
// the Cacher will execute the closure at that time and store the result within
// a Some variant in the value field. Then if the code asks for the result of the
// closure again, instead of executing the closure again, the Cacher will return
// the result held in the Some variant.
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout4(intensity: u32, random_number: u32) {
    // We want Cacher to manage the struct fields’ values rather than letting the
    // calling code potentially change the values in these fields directly, so these
    // fields are private.
    // The Cacher::new function takes a generic parameter T, which we’ve defined as
    // having the same trait bound as the Cacher struct. Then Cacher::new returns a
    // Cacher instance that holds the closure specified in the calculation field and
    // a None value in the value field, because we haven’t executed the closure yet.
    // When the calling code needs the result of evaluating the closure, instead of
    // calling the closure directly, it will call the value method. This method
    // checks whether we already have a resulting value in self.value in a Some;
    // if we do, it returns the value within the Some without executing the closure again.
    // If self.value is None, the code calls the closure stored in self.calculation,
    // saves the result in self.value for future use, and returns the value as well.
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}

// Limitations of the Cacher Implementation
// Caching values is a generally useful behavior that we might want to use in
// other parts of our code with different closures. However, there are two
// problems with the current implementation of Cacher that would make reusing
// it in different contexts difficult.
#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);
    let _v1 = c.value(1);
    // The first problem is that a Cacher instance assumes it will always
    // get the same value for the parameter arg to the value method.
    assert_eq!(c.value(2), 1);
}
// The second problem with the current Cacher implementation is that it only
// accepts closures that take one parameter of type u32 and return a u32.
// We might want to cache the results of closures that take a string slice
// and return usize values, for example. To fix this issue, try introducing
// more generic parameters to increase the flexibility of the Cacher functionality.
