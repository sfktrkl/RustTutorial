use std::collections::HashSet;

fn main() {
    let vowels = HashSet::<char>::from(['a', 'e', 'i', 'o', 'u']);
    let words = vec!["first", "apple", "climax", "random", "enigma", "word"];
    let mut pig_latin = Vec::<String>::new();
    for word in words {
        let ch = word.chars().next().unwrap();
        let mut new = String::new();
        if !vowels.contains(&ch) {
            new.push_str(&word[1..]);
            new.push_str("-");
            new.push(ch);
            new.push_str("ay");
        } else {
            new.push_str(word);
            new.push_str("-hay");
        }
        pig_latin.push(new);
    }
    println!("{:?}", pig_latin);
}
