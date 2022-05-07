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
