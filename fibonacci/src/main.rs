use std::io;

fn main() {
    loop {
        println!("nth Fibonacci number.");

        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");

        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{}", fib(number));
    }
}

fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}
