// Our two crates are tightly related, so we create the procedural macro crate
// within the directory of our hello_macro crate. If we change the trait
// definition in hello_macro, we’ll have to change the implementation of the
// procedural macro in hello_macro_derive as well. The two crates will need to
// be published separately, and programmers using these crates will need to add
// both as dependencies and bring them both into scope. We could instead have
// the hello_macro crate use hello_macro_derive as a dependency and re-export
// the procedural macro code. However, the way we’ve structured the project
// makes it possible for programmers to use hello_macro even if they don’t want
// the derive functionality.
use proc_macro::TokenStream;

// We need to declare the hello_macro_derive crate as a procedural macro crate.
// We’ll also need functionality from the syn and quote crates, as you’ll see in
// a moment, so we need to add them as dependencies.

// We’ve introduced three new crates: proc_macro, syn, and quote. The proc_macro
// crate comes with Rust, so we didn’t need to add that to the dependencies in
// Cargo.toml. The proc_macro crate is the compiler’s API that allows us to read
// and manipulate Rust code from our code.

// The syn crate parses Rust code from a string into a data structure that we
// can perform operations on. The quote crate turns syn data structures back
// into Rust code. These crates make it much simpler to parse any sort of Rust
// code we might want to handle: writing a full parser for Rust code is no
// simple task.
use quote::quote;
use syn;

// Notice that we’ve split the code into the hello_macro_derive function, which
// is responsible for parsing the TokenStream, and the impl_hello_macro
// function, which is responsible for transforming the syntax tree: this makes
// writing a procedural macro more convenient. The code in the outer function
// (hello_macro_derive in this case) will be the same for almost every
// procedural macro crate you see or create. The code you specify in the body of
// the inner function (impl_hello_macro in this case) will be different
// depending on your procedural macro’s purpose.

// The hello_macro_derive function will be called when a user of our library
// specifies #[derive(HelloMacro)] on a type. This is possible because we’ve
// annotated the hello_macro_derive function here with proc_macro_derive and
// specified the name, HelloMacro, which matches our trait name; this is the
// convention most procedural macros follow.
#[proc_macro_derive(HelloMacro)]
// The hello_macro_derive function first converts the input from a TokenStream
// to a data structure that we can then interpret and perform operations on.
// This is where syn comes into play. The parse function in syn takes a
// TokenStream and returns a DeriveInput struct representing the parsed Rust
// code.
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree that we can
    // manipulate

    // You might have noticed that we’re calling unwrap to cause the
    // hello_macro_derive function to panic if the call to the syn::parse
    // function fails here. It’s necessary for our procedural macro to panic on
    // errors because proc_macro_derive functions must return TokenStream rather
    // than Result to conform to the procedural macro API. We’ve simplified this
    // example by using unwrap; in production code, you should provide more
    // specific error messages about what went wrong by using panic! or expect.
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

// Soon we’ll define the impl_hello_macro function, which is where we’ll build
// the new Rust code we want to include. But before we do, note that the output
// for our derive macro is also a TokenStream. The returned TokenStream is added
// to the code that our crate users write, so when they compile their crate,
// they’ll get the extra functionality that we provide in the modified
// TokenStream.
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // We get an Ident struct instance containing the name (identifier) of the
    // annotated type using ast.ident. when we run the impl_hello_macro function
    // the ident we get will have the ident field with a value of "Pancakes".
    // Thus, the name variable in Listing 19-33 will contain an Ident struct
    // instance that, when printed, will be the string "Pancakes".
    let name = &ast.ident;
    // The quote! macro lets us define the Rust code that we want to return. The
    // compiler expects something different to the direct result of the quote!
    // macro’s execution, so we need to convert it to a TokenStream. We do this
    // by calling the into method, which consumes this intermediate
    // representation and returns a value of the required TokenStream type.
    let gen = quote! {
        // The quote! macro also provides some very cool templating mechanics: we
        // can enter #name, and quote! will replace it with the value in the
        // variable name. You can even do some repetition similar to the way regular
        // macros work.
        impl HelloMacro for #name {
            // We want our procedural macro to generate an implementation of our
            // HelloMacro trait for the type the user annotated, which we can
            // get by using #name. The trait implementation has one function,
            // hello_macro, whose body contains the functionality we want to
            // provide: printing Hello, Macro! My name is and then the name of
            // the annotated type.
            fn hello_macro() {
                // The stringify! macro used here is built into Rust. It takes a
                // Rust expression, such as 1 + 2, and at compile time turns the
                // expression into a string literal, such as "1 + 2". This is
                // different than format! or println!, macros which evaluate the
                // expression and then turn the result into a String. There is a
                // possibility that the #name input might be an expression to
                // print literally, so we use stringify!. Using stringify! also
                // saves an allocation by converting #name to a string literal
                // at compile time.
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
