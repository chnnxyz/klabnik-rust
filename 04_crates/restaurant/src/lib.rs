// The mod keyword defines a module and its name
mod front_of_house {
    // The hosting and serving mods are submodules
    pub mod hosting {
        //modules can contain structs, enums, constants, traits or functions
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    fn cook_order() {}

    fn fix_incorrect_order() {
        // cook_order lives in the same context, so 
        // access is direct
        cook_order();
        // Super will take you to the parent module
        // this works, obvs, only for rel paths
        super::front_of_house::serving::serve_order();
    }

    //let's make a breakfast struct
    pub struct Breakfast{
        pub toast: String,
        seasonal_fruit: String,
    }

    //and add in some methods
    impl Breakfast{
        // summer breakfast, apply method on breakfast variable
        // to get a new breakfasr
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        pub fn winter(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("blueberry"),
            }
        }
    }

    //for structs, all properties need to have their privacy assigned
    //enum variants inherit privacy from the enum
    pub enum Appetizer {
        Soup,
        Salad,
    }
}
//:: is equivalent to . in python modules
// public functions in the api require a pub keyword

// we can add modules into the scope with use
use crate::front_of_house::hosting;
// we can also use a relative path in use

// Commonly we use call in modules, not individual functions,
// unless we are calling in structs, consts or enums
// Just like in python, we can use the as keyword for renaming
// our modules in the code

// using an undefined mod will look for a .rs file with same name



pub fn eat_at_restaurant() {
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    //front_of_house::hosting::add_to_waitlist();

    // We can then start our path to add to waitlist with the
    // Module name
    hosting::add_to_waitlist();

    //Create a summer breakfast with rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    //Since toast is public, it can be changed.
    meal.toast = String::from("Wheat");
    println!("Some {} toast please", meal.toast);

    //since the seasonal fruit is private, we cannot change it so
    // meal.seasonal_fruit::from("apples");
    // won't compile

    //Let's verify the enums

    let order1 = back_of_house::Appetizer::Soup;
}

// Privacy:
// Items in a parent module can't use private items in child modules
// child modules can use items in ancestor modules.
// Techinically, the child can see the context of the parents
// Sibling folders can access each other by name, so using
// abs path will not be a problem for fn eat_at_restaurant()
// paths brought into scope also check privacy

// Library files have dedicated unit test sections using  #[cfg(test)]
// and #[test]
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
