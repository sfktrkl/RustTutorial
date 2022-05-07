fn main() {
    // The Twelve Days of Christmas
    let numbers = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    for number in 1..12 {
        println!("On the {} day of Christmas", numbers[number]);
        println!("My good friends brought to me");
        if number > 11 {
            println!("All their good wishes");
        }
        if number > 10 {
            println!("Gifts for one and all");
        }
        if number > 9 {
            println!("Some mistletoe");
        }
        if number > 8 {
            println!("A guardian angel");
        }
        if number > 7 {
            println!("Gold and silver tinsel");
        }
        if number > 6 {
            println!("Candles a-glowing");
        }
        if number > 5 {
            println!("Little silver bells");
        }
        if number > 4 {
            println!("A shining star");
        }
        if number > 3 {
            println!("Four colored lights");
        }
        if number > 2 {
            println!("Three boughs of holly");
        }
        if number > 1 {
            println!("Two candy canes");
        }
        if number == 1 {
            println!("A song and a Christmas tree");
        } else {
            println!("And A song and a Christmas tree");
        }
        println!("");
    }
}
