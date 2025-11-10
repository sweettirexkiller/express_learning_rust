mod front_of_house;

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

pub fn eat_at_restaurant() {
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // // Relative path
    // front_of_house::hosting::add_to_waitlist();

    // let mut meal = back_of_house::Breakfast::summer("Rye");

    // meal.toast = String::from("Wheat");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;


}

fn serve_order() {}

pub use crate::front_of_house::hosting;

use rand::{Rng, thread_rng, CryptoRng};

// use std::io{self, Write};
use std::io:::*;

pub fn eat_at_restaurant2() {
    hosting::add_to_waitlist();
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is: {}", secret_number);
}

// use std::fmt::Res;
// use std::io::Result as IoResult;
