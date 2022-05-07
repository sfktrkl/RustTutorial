use std::io;

fn main() {
    loop {
        println!("Enter temperature in fahrenheit (in degrees).");

        let mut fahrenheit = String::new();
        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read line");

        let fahrenheit: f64 = match fahrenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{}", fahrenheit_to_celsius(fahrenheit));
    }
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
