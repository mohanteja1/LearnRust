pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

fn serve_order() {}

mod back_of_house {

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

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
    fn cook_order() {}

    pub enum Appetizer {
        Soup,
        Salad,
    }
}


fn eat_at_restaurant() {
    // order some breakfast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change our minde about the type of toast
    meal.toast = String::from("wheat");
    print!("toast type: {}", meal.toast);
    // you cannot change the seasonal fruit used: ie.. peaches
    // meal.seasonal_fruit = String::from("apple"); 
    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;
}

pub fn eat_at_house() {
    // absolute path: 
    crate::front_of_house::hosting::add_to_waitlist();
    // relative path: 
    front_of_house::hosting::add_to_waitlist(); 
}

use self::front_of_house::hosting;

pub fn eat_at_restaurant_second_time() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

use self::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant_third_time() {
    add_to_waitlist();
    add_to_waitlist();
}


use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    fmt::Result::Ok(())
}

fn function2() -> io::Result<()> {
    io::Result::Ok(())
}

// reexporting 
// pub use crate::front_of_house::hosting;


// moving creating modules : 
// check the file : top_of_house.rs
mod top_of_house;

pub fn eat_at_restaurant_fourth_time() {
    front_of_house::hosting::add_to_waitlist();
    let breakfast = back_of_house::Breakfast::summer("wheat");
    top_of_house::lights::turn_off_lights();
    top_of_house::fire_works::burst_crackers();
    top_of_house::lights::turn_on_festival_decoration_lights();
    top_of_house::lights::turn_on_hotel_name_lights();
}

