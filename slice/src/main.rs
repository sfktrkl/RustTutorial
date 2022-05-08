fn main() {
    // Slices let you reference a contiguous sequence of elements in
    // a collection rather than the whole collection.
    // A slice is a kind of reference, so it does not have ownership.
    let mut s = String::from("hello world");
    let _word = first_word(&s); // word will get the value 5
    s.clear(); // this empties the String, making it equal to ""
               // word still has the value 5 here, but there's no more string that
               // we could meaningfully use the value 5 with. word is now totally invalid!

    // String Slices
    // A string slice is a reference to part of a String, and it looks like this.
    // Rather than a reference to the entire String,
    // hello is a reference to a portion of the String,
    // specified in the extra [0..5] bit.
    // We create slices using a range within brackets by specifying
    // [starting_index..ending_index], where starting_index is the first
    // position in the slice and ending_index is one more than
    // the last position in the slice.
    let s = String::from("hello world");
    let _hello = &s[0..5];
    let _world = &s[6..11];

    // With Rust’s .. range syntax, if you want to start at index zero,
    // you can drop the value before the two periods.
    let s = String::from("hello");
    let _slice = &s[0..2];
    let _slice = &s[..2];

    //By the same token, if your slice includes the last byte of the String,
    // you can drop the trailing number.
    let s = String::from("hello");
    let len = s.len();
    let _slice = &s[3..len];
    let _slice = &s[3..];

    // You can also drop both values to take a slice of the entire string.
    let s = String::from("hello");
    let len = s.len();
    let _slice = &s[0..len];
    let _slice = &s[..];

    // Recall from the borrowing rules that if we have an immutable reference
    // to something, we cannot also take a mutable reference.
    // Because clear needs to truncate the String, it needs to get a mutable
    // reference. The println! after the call to clear uses the reference
    // in word, so the immutable reference must still be active at that point.
    // Rust disallows the mutable reference in clear and the immutable reference
    // in word from existing at the same time, and compilation fails.
    let /*mut*/ s = String::from("hello world");
    let word = first_word_re(&s);
    //s.clear(); // error!
    println!("the first word is: {}", word);

    // String Literals Are Slices
    // Recall that we talked about string literals being stored inside the binary.
    // Now that we know about slices.
    let _s = "Hello, world!";

    // String Slices as Parameters
    // If we have a string slice, we can pass that directly.
    // If we have a String, we can pass a slice of the String or
    // a reference to the String.
    {
        let my_string = String::from("hello world");
        // `first_word` works on slices of `String`s, whether partial or whole
        let _word = first_word_sig(&my_string[0..6]);
        let _word = first_word_sig(&my_string[..]);
        // `first_word` also works on references to `String`s, which are equivalent
        // to whole slices of `String`s
        let _word = first_word_sig(&my_string);
        let my_string_literal = "hello world";
        // `first_word` works on slices of string literals, whether partial or whole
        let _word = first_word_sig(&my_string_literal[0..6]);
        let _word = first_word_sig(&my_string_literal[..]);
        // Because string literals *are* string slices already,
        // this works too, without the slice syntax!
        let _word = first_word_sig(my_string_literal);
    }

    // Other Slices
    // String slices, as you might imagine, are specific to strings.
    // But there’s a more general slice type, too.
    // Just as we might want to refer to a part of a string,
    // we might want to refer to part of an array.
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

// The first_word function has a &String as a parameter.
// We don’t want ownership, so this is fine.
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// With all this information in mind, let’s rewrite first_word to return a slice.
fn first_word_re(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Knowing that you can take slices of literals and String values leads
// us to one more improvement on first_word, and that’s its signature.
// A more experienced Rustacean would write the signature shown in Listing 4-9
// instead because it allows us to use the same function on both
// &String values and &str values.
fn first_word_sig(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
