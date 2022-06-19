# RustTutorial
https://doc.rust-lang.org/stable/book/

## Creating a Project with Cargo

```cmd
cargo new hello_cargo
cd hello_cargo
```

## Building and Running a Cargo Project

* We can build a project using cargo build.
* We can build and run a project in one step using cargo run.
* We can build a project without producing a binary to check for errors using cargo check.
* Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.

```cmd
cargo build
.\target\debug\hello_cargo.exe
cargo run
cargo check
```

## Building for Release

```cmd
cargo build --release
```

## Format Rust

```cmd
rustfmt
cargo fmt
```

## Using a Crate to Get More Functionality

Remember that a crate is a collection of Rust source code files. The project we’ve been building is a binary crate, which is an executable. The rand crate is a library crate, which contains code intended to be used in other programs, and can’t be executed on its own.

Open `Cargo.toml` file now and add the following line to the bottom beneath the [dependencies] section header that Cargo created for you.
```
rand = "0.8.3"
```

Crates.io is where people in the Rust ecosystem post their open source Rust projects for others to use.

## Updating a Crate to Get a New Version

When you do want to update a crate, Cargo provides the command update, which will ignore the Cargo.lock file and figure out all the latest versions that fit your specifications in Cargo.toml.
```cmd
cargo update
```

## Packages and Crates

The first parts of the module system we’ll cover are packages and crates. A crate is a binary or library. The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate. A package is one or more crates that provide a set of functionality. A package contains a Cargo.toml file that describes how to build those crates.
```cmd
cargo new my-project
ls my-project
ls my-project/src
```

Here, we have a package that only contains src/main.rs, meaning it only contains a binary crate named my-project. If a package contains src/main.rs and src/lib.rs, it has two crates: a binary and a library, both with the same name as the package. A package can have multiple binary crates by placing files in the src/bin directory: each file will be a separate binary crate.

A crate will group related functionality together in a scope so the functionality is easy to share between multiple projects.

## Defining Modules to Control Scope and Privacy

Modules let us organize code within a crate into groups for readability and easy reuse. Modules also control the privacy of items, which is whether an item can be used by outside code (public) or is an internal implementation detail and not available for outside use (private).

Create a new library named restaurant by running
```cmd
cargo new --lib restaurant
```

## Unrecoverable Errors with panic!

Sometimes, bad things happen in your code, and there’s nothing you can do about it. In these cases, Rust has the panic! macro. When the panic! macro executes, your program will print a failure message, unwind and clean up the stack, and then quit.

By default, when a panic occurs, the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters. However, this walking back and cleanup is a lot of work. Rust, therefore, allows you to choose the alternative of immediately aborting, which ends the program without cleaning up.
```rust
panic!("crash and burn");
```

If in your project you need to make the resulting binary as small as possible, you can switch from unwinding to aborting upon a panic by adding panic = 'abort' to the appropriate [profile] sections in your Cargo.toml file.
```toml
[profile.release]
panic = 'abort'
```

A backtrace is a list of all the functions that have been called to get to this point. Backtraces in Rust work as they do in other languages: the key to reading the backtrace is to start from the top and read until you see files you wrote.

Let’s try getting a backtrace by setting the RUST_BACKTRACE environment variable to any value except 0.
```cmd
RUST_BACKTRACE=1 cargo run
```

## To panic! or Not to panic!

So how do you decide when you should call panic! and when you should return Result? When code panics, there’s no way to recover. You could call panic! for any error situation, whether there’s a possible way to recover or not, but then you’re making the decision that a situation is unrecoverable on behalf of the calling code. When you choose to return a Result value, you give the calling code options. The calling code could choose to attempt to recover in a way that’s appropriate for its situation, or it could decide that an Err value in this case is unrecoverable, so it can call panic! and turn your recoverable error into an unrecoverable one. Therefore, returning Result is a good default choice when you’re defining a function that might fail.

## Examples, Prototype Code, and Tests

The unwrap and expect methods are very handy when prototyping, before you’re ready to decide how to handle errors. They leave clear markers in your code for when you’re ready to make your program more robust.

## Guidelines for Error Handling

It’s advisable to have your code panic when it’s possible that your code could end up in a bad state. In this context, a bad state is when some assumption, guarantee, contract, or invariant has been broken, such as when invalid values, contradictory values, or missing values are passed to your code—plus one or more of the following:
* The bad state is something that is unexpected, as opposed to something that will likely happen occasionally, like a user entering data in the wrong format.
* Your code after this point needs to rely on not being in this bad state, rather than checking for the problem at every step.
* There’s not a good way to encode this information in the types you use. We’ll work through an example of what we mean in the “Encoding States and Behavior as Types” section of Chapter 17.

If someone calls your code and passes in values that don’t make sense, the best choice might be to call panic! and alert the person using your library to the bug in their code so they can fix it during development. Similarly, panic! is often appropriate if you’re calling external code that is out of your control and it returns an invalid state that you have no way of fixing.

## Creating Custom Types for Validation

Let’s take the idea of using Rust’s type system to ensure we have a valid value one step further and look at creating a custom type for validation. Recall the guessing game in Chapter 2 in which our code asked the user to guess a number between 1 and 100. We never validated that the user’s guess was between those numbers before checking it against our secret number; we only validated that the guess was positive. In this case, the consequences were not very dire: our output of “Too high” or “Too low” would still be correct. But it would be a useful enhancement to guide the user toward valid guesses and have different behavior when a user guesses a number that’s out of range versus when a user types, for example, letters instead.
```rust
loop {
    // --snip--

    let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    if guess < 1 || guess > 100 {
        println!("The secret number will be between 1 and 100.");
        continue;
    }

    match guess.cmp(&secret_number) {
        // --snip--
}
```
However, this is not an ideal solution: if it was absolutely critical that the program only operated on values between 1 and 100, and it had many functions with this requirement, having a check like this in every function would be tedious (and might impact performance).

Instead, we can make a new type and put the validations in a function to create an instance of the type rather than repeating the validations everywhere.

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```
First, we define a struct named Guess that has a field named value that holds an i32. This is where the number will be stored.

Then we implement an associated function named new on Guess that creates instances of Guess values. The new function is defined to have one parameter named value of type i32 and to return a Guess. The code in the body of the new function tests value to make sure it’s between 1 and 100. If value doesn’t pass this test, we make a panic! call, which will alert the programmer who is writing the calling code that they have a bug they need to fix, because creating a Guess with a value outside this range would violate the contract that Guess::new is relying on.

## Creating a Workspace

A workspace is a set of packages that share the same Cargo.lock and output directory. Let’s make a project using a workspace—we’ll use trivial code so we can concentrate on the structure of the workspace. There are multiple ways to structure a workspace; we’re going to show one common way. We’ll have a workspace containing a binary and two libraries. The binary, which will provide the main functionality, will depend on the two libraries. One library will provide an add_one function, and a second library an add_two function. These three crates will be part of the same workspace. We’ll start by creating a new directory for the workspace:
```cmd
mkdir workspaces
cd workspaces
```

Next, in the add directory, we create the Cargo.toml file that will configure the entire workspace. This file won’t have a [package] section or the metadata we’ve seen in other Cargo.toml files. Instead, it will start with a [workspace] section that will allow us to add members to the workspace by specifying the path to the package with our binary crate; in this case, that path is adder:
```toml
[workspace]
members = [
    "adder",
]
```

Next, we’ll create the adder binary crate by running cargo new within the add directory:
```cmd
cargo new adder
```

At this point, we can build the workspace by running cargo build. The files in your add directory should look like this:
```
├── Cargo.lock
├── Cargo.toml
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

The workspace has one target directory at the top level for the compiled artifacts to be placed into; the adder package doesn’t have its own target directory. Even if we were to run cargo build from inside the adder directory, the compiled artifacts would still end up in add/target rather than add/adder/target. Cargo structures the target directory in a workspace like this because the crates in a workspace are meant to depend on each other. If each crate had its own target directory, each crate would have to recompile each of the other crates in the workspace to have the artifacts in its own target directory. By sharing one target directory, the crates can avoid unnecessary rebuilding.

## Creating the Second Package in the Workspace

Next, let’s create another member package in the workspace and call it add_one. Change the top-level Cargo.toml to specify the add_one path in the members list:
```toml
[workspace]
members = [
    "adder",
    "add_one",
]
```

Then generate a new library crate named add_one:
```cmd
cargo new add_one --lib
```

Your add directory should now have these directories and files:
```
├── Cargo.lock
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

Now that we have another package in the workspace, we can have the adder package with our binary depend on the add_one package, that has our library. First, we’ll need to add a path dependency on add_one to adder/Cargo.toml.

Cargo doesn’t assume that crates in a workspace will depend on each other, so we need to be explicit about the dependency relationships between the crates.

Next, let’s use the add_one function from the add_one crate in the adder crate. Open the adder/src/main.rs file and add a use line at the top to bring the new add_one library crate into scope.

Let’s build the workspace by running cargo build in the top-level add directory!

## Depending on an External Package in a Workspace

Notice that the workspace has only one Cargo.lock file at the top level of the workspace rather than having a Cargo.lock in each crate’s directory. This ensures that all crates are using the same version of all dependencies. If we add the rand package to the adder/Cargo.toml and add_one/Cargo.toml files, Cargo will resolve both of those to one version of rand and record that in the one Cargo.lock. Making all crates in the workspace use the same dependencies means the crates in the workspace will always be compatible with each other. Let’s add the rand crate to the [dependencies] section in the add_one/Cargo.toml file to be able to use the rand crate in the add_one crate:
```toml
[dependencies]
rand = "0.8.3"
```

We can now add use rand; to the add_one/src/lib.rs file, and building the whole workspace by running cargo build in the add directory will bring in and compile the rand crate. We will get one warning because we aren’t referring to the rand we brought into scope:
```cmd
cargo build
```
