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