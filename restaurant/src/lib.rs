#![allow(dead_code)]

// We define a module by starting with the mod keyword and then specify
// the name of the module (in this case, front_of_house) and place curly
// brackets around the body of the module. Inside modules, we can have other
// modules, as in this case with the modules hosting and serving.

// By using modules, we can group related definitions together and name
// why they’re related. Programmers using this code would have an easier
// time finding the definitions they wanted to use because they could navigate
// the code based on the groups rather than having to read through
// all the definitions. Programmers adding new functionality to this
// code would know where to place the code to keep the program organized.
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
