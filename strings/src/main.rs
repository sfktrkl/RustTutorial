fn main() {
    // What Is a String?
    // We’ll first define what we mean by the term string. Rust has only one string
    // type in the core language, which is the string slice str that is usually
    // seen in its borrowed form &str.
    // The String type, which is provided by Rust’s standard library rather
    // than coded into the core language, is a growable, mutable, owned,
    // UTF-8 encoded string type.

    // Creating a New String
    // Many of the same operations available with Vec<T> are available
    // with String as well, starting with the new function to create a string.
    // This line creates a new empty string called s, which we can then load data into.
    let mut _s = String::new();

    // Often, we’ll have some initial data that we want to start the string with.
    // For that, we use the to_string method, which is available on any type
    // that implements the Display trait, as string literals do.
    let data = "initial contents";
    let _s = data.to_string();

    // the method also works on a literal directly:
    let _s = "initial contents".to_string();

    // We can also use the function String::from to create a String from a string literal.
    let _s = String::from("initial contents");

    // Updating a String
    // A String can grow in size and its contents can change, just like the
    // contents of a Vec<T>, if you push more data into it. In addition,
    // you can conveniently use the + operator or the format! macro to
    // concatenate String values.

    // Appending to a String with push_str and push
    // We can grow a String by using the push_str method to append a string slice.
    // After these two lines, s will contain foobar. The push_str method takes
    // a string slice because we don’t necessarily want to take ownership of the parameter.
    let mut s = String::from("foo");
    s.push_str("bar");

    // If the push_str method took ownership of s2, we wouldn’t be able to
    // print its value on the last line. However, this code works as we’d expect!
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    // The push method takes a single character as a parameter and adds it to the String.
    let mut s = String::from("lo");
    s.push('l');

    // Concatenation with the + Operator or the format! Macro
    // Often, you’ll want to combine two existing strings.
    // One way to do so is to use the + operator.
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let _s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // The + operator uses the add method, whose signature looks something like this:
    // fn add(self, s: &str) -> String {
    // s2 has an &, meaning that we’re adding a reference of the second string to the first string.
    // Add does not take ownership of the s parameter, s2 will still be a valid
    // String after this operation.
    // We can see in the signature that add takes ownership of self,
    // because self does not have an &. This means s1 will be moved into the add
    // call and will no longer be valid after that.
    // So although let s3 = s1 + &s2; looks like it will copy both strings and
    // create a new one, this statement actually takes ownership of s1,
    // appends a copy of the contents of s2, and then returns ownership of the result.

    // If we need to concatenate multiple strings,
    // the behavior of the + operator gets unwieldy.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let _s = s1 + "-" + &s2 + "-" + &s3;

    // For more complicated string combining, we can instead use the format! macro.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let _s = format!("{}-{}-{}", s1, s2, s3);

    // Indexing into Strings
    // In many other programming languages, accessing individual characters
    // in a string by referencing them by index is a valid and common operation.
    // However, if you try to access parts of a String using indexing syntax
    // in Rust, you’ll get an error.
    let _s1 = String::from("hello");
    //let h = _s1[0];

    // Internal Representation
    // A String is a wrapper over a Vec<u8>.
    // In this case, len will be 4, which means the vector storing the string
    // “Hola” is 4 bytes long.
    let _hello = String::from("Hola");

    // Asked how long the string is, you might say 12. In fact, Rust’s answer
    // is 24: that’s the number of bytes it takes to encode “Здравствуйте” in
    // UTF-8, because each Unicode scalar value in that string takes 2 bytes
    // of storage. Therefore, an index into the string’s bytes will not always
    // correlate to a valid Unicode scalar value.
    let _hello = String::from("Здравствуйте");

    // Bytes and Scalar Values and Grapheme Clusters! Oh My!
    // Another point about UTF-8 is that there are actually three relevant ways
    // to look at strings from Rust’s perspective: as bytes, scalar values,
    // and grapheme clusters.
    // If we look at the Hindi word “नमस्ते” written in the Devanagari script,
    // it is stored as a vector of u8 values that looks like this.
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
    // 224, 165, 135]
    // That’s 18 bytes and is how computers ultimately store this data.
    // If we look at them as Unicode scalar values, which are what Rust’s char
    // type is, those bytes look like this.
    // ['न', 'म', 'स', '्', 'त', 'े']
    // There are six char values here, but the fourth and sixth are not letters:
    // they’re diacritics that don’t make sense on their own.
    // Finally, if we look at them as grapheme clusters.
    // ["न", "म", "स्", "ते"]
    // A final reason Rust doesn’t allow us to index into a String to get
    // a character is that indexing operations are expected to always take
    // constant time (O(1)).

    // Slicing Strings
    // Indexing into a string is often a bad idea because it’s not clear what
    // the return type of the string-indexing operation should be: a byte value,
    // a character, a grapheme cluster, or a string slice. If you really need to
    // use indices to create string slices, therefore, Rust asks you to be more specific.
    // Rather than indexing using [] with a single number, you can use [] with
    // a range to create a string slice containing particular bytes:
    let hello = "Здравствуйте";
    // Here, s will be a &str that contains the first 4 bytes of the string.
    // Earlier, we mentioned that each of these characters was 2 bytes,
    // which means s will be Зд.
    // If we were to try to slice only part of a character’s bytes with
    // something like &hello[0..1], Rust would panic at runtime in the
    // same way as if an invalid index were accessed in a vector.
    let _s = &hello[0..4];

    // Methods for Iterating Over Strings
    // The best way to operate on pieces of strings is to be explicit about
    // whether you want characters or bytes. For individual Unicode scalar
    // values, use the chars method.
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    // Alternatively, the bytes method returns each raw byte,
    // which might be appropriate for your domain.
    // But be sure to remember that valid Unicode scalar values
    // may be made up of more than 1 byte.
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}
