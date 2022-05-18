// Note that we need to first use the HashMap from the collections portion
// of the standard library. Of our three common collections, this one is the
// least often used, so it’s not included in the features brought into scope
// automatically in the prelude.
use std::collections::HashMap;

fn main() {
    // Creating a New Hash Map
    // One way to create an empty hash map is using new and adding elements with insert.
    // Just like vectors, hash maps store their data on the heap. This HashMap
    // has keys of type String and values of type i32. Like vectors, hash maps
    // are homogeneous: all of the keys must have the same type, and all of
    // the values must have the same type.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Another way of constructing a hash map is by using iterators and the
    // collect method on a vector of tuples, where each tuple consists of
    // a key and its value.
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    // The type annotation HashMap<_, _> is needed here because it’s possible
    // to collect into many different data structures and Rust doesn’t know
    // which you want unless you specify. For the parameters for the key and
    // value types, however, we use underscores, and Rust can infer the types
    // that the hash map contains based on the types of the data in the vectors.
    let mut _scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    // Hash Maps and Ownership
    // For types that implement the Copy trait, like i32, the values are copied
    // into the hash map. For owned values like String, the values will be moved
    // and the hash map will be the owner of those values.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // Accessing Values in a Hash Map
    // We can get a value out of the hash map by providing its key to the get method.
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // Here, score will have the value that’s associated with the Blue team,
    // and the result will be Some(&10). The result is wrapped in Some because
    // get returns an Option<&V>; if there’s no value for that key
    // in the hash map, get will return None.
    let _score = scores.get(&team_name);

    // We can iterate over each key/value pair in a hash map in a similar
    // manner as we do with vectors, using a for loop.
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Updating a Hash Map
    // Although the number of key and value pairs is growable,
    // each key can only have one value associated with it at a time.
    // When you want to change the data in a hash map, you have to decide
    // how to handle the case when a key already has a value assigned.
    // You could replace the old value with the new value, completely
    // disregarding the old value. You could keep the old value and ignore
    // the new value, only adding the new value if the key doesn’t already
    // have a value. Or you could combine the old value and the new value.
    // Let’s look at how to do each of these!

    // Overwriting a Value
    // If we insert a key and a value into a hash map and then insert
    // that same key with a different value, the value associated with
    // that key will be replaced.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("{:?}", scores);

    // Only Inserting a Value If the Key Has No Value
    // It’s common to check whether a particular key has a value and,
    // if it doesn’t, insert a value for it. Hash maps have a special
    // API for this called entry that takes the key you want to check
    // as a parameter. The return value of the entry method is an enum
    // called Entry that represents a value that might or might not exist.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    // Updating a Value Based on the Old Value
    // Another common use case for hash maps is to look up a key’s value
    // and then update it based on the old value.
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // The or_insert method returns a mutable reference (&mut V) to the value
        // for the specified key. Here we store that mutable reference in
        // the count variable, so in order to assign to that value, we must
        // first dereference count using the asterisk (*).
        // The mutable reference goes out of scope at the end of the for loop,
        // so all of these changes are safe and allowed by the borrowing rules.
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    // Hashing Functions
    // By default, HashMap uses a hashing function called SipHash that can
    // provide resistance to Denial of Service (DoS) attacks involving hash
    // tables. This is not the fastest hashing algorithm available, but the
    // trade-off for better security that comes with the drop in performance
    // is worth it. If you profile your code and find that the default hash
    // function is too slow for your purposes, you can switch to another
    // function by specifying a different hasher.
}
