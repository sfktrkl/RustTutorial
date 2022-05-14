#![allow(dead_code)]

// To show Rust where to find an item in a module tree, we use a path
// in the same way we use a path when navigating a filesystem.
// If we want to call a function, we need to know its path.
// A path can take two forms:
//  An absolute path starts from a crate root by using a crate name or a literal crate.
//  A relative path starts from the current module and uses self,
//      super, or an identifier in the current module.

mod front_of_house {
    // Exposing Paths with the pub Keyword
    // We want the eat_at_restaurant function in the parent module to have
    // access to the add_to_waitlist function in the child module,
    // so we mark the hosting module with the pub keyword.
    pub mod hosting {
        // Let’s also make the add_to_waitlist function public by
        // adding the pub keyword before its definition.
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Both absolute and relative paths are followed by one or more identifiers
    // separated by double colons (::).

    // Absolute path
    // In the absolute path, we start with crate, the root of our crate’s
    // module tree. Then the front_of_house module is defined in the crate root.
    // The front_of_house module isn’t public, but because the eat_at_restaurant
    // function is defined in the same module as front_of_house
    // we can refer to front_of_house from eat_at_restaurant.
    // Next is the hosting module marked with pub. We can access
    // the parent module of hosting, so we can access hosting.
    // Finally, the add_to_waitlist function is marked with pub and
    // we can access its parent module, so this function call works!
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // In the relative path, the logic is the same as the absolute path except
    // for the first step: rather than starting from the crate root, the path
    // starts from front_of_house. The front_of_house module is defined within
    // the same module as eat_at_restaurant, so the relative path starting from
    // the module in which eat_at_restaurant is defined works. Then, because
    // hosting and add_to_waitlist are marked with pub, the rest of the
    // path works, and this function call is valid!
    front_of_house::hosting::add_to_waitlist();

    // Choosing whether to use a relative or absolute path is a decision
    // you’ll make based on your project. The decision should depend on
    // whether you’re more likely to move item definition code separately
    // from or together with the code that uses the item.

    // Modules aren’t useful only for organizing your code.
    // They also define Rust’s privacy boundary: the line that encapsulates
    // the implementation details external code isn’t allowed to know about,
    // call, or rely on. So, if you want to make an item like a function or
    // struct private, you put it in a module.

    // The way privacy works in Rust is that all items
    // (functions, methods, structs, enums, modules, and constants)
    // are private by default. Items in a parent module can’t use the
    // private items inside child modules, but items in child modules
    // can use the items in their ancestor modules.
}

// Starting Relative Paths with super
// We can also construct relative paths that begin in the parent module
// by using super at the start of the path.
// This is like starting a filesystem path with the .. syntax.
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();

        // The fix_incorrect_order function is in the back_of_house module,
        // so we can use super to go to the parent module of back_of_house,
        // which in this case is crate, the root.
        // From there, we look for serve_order and find it.
        super::serve_order();
    }

    fn cook_order() {}
}

// Making Structs and Enums Public
// We can also use pub to designate structs and enums as public,
// but there are a few extra details. If we use pub before a struct definition,
// we make the struct public, but the struct’s fields will still be private.
// We can make each field public or not on a case-by-case basis.
mod back_of_house2 {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant2() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house2::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

// In contrast, if we make an enum public, all of its variants are then public.
// We only need the pub before the enum keyword.
mod back_of_house3 {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant3() {
    let _order1 = back_of_house3::Appetizer::Soup;
    let _order2 = back_of_house3::Appetizer::Salad;
}

// Because we made the Appetizer enum public, we can use the Soup and Salad
// variants in eat_at_restaurant. Enums aren’t very useful unless their
// variants are public; it would be annoying to have to annotate all
// enum variants with pub in every case, so the default for enum variants
// is to be public. Structs are often useful without their fields being public,
// so struct fields follow the general rule of everything being private
// by default unless annotated with pub.
