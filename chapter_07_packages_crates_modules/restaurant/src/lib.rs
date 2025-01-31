pub use crate::front_of_house::hosting;

mod front_of_house;

mod back_of_house {
    pub struct Breakfast { // Making the struct public doesn't make its fields public
        pub toast: String, // It is necessary to make declared fields public as well
        seasonal_fruit: String,
    }

    pub enum Appetizer { // Making the enum public makes all its variants public as well
        Soup,
        Salad,
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

pub fn eat_at_restaurant() {
    // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();
    // // Relative path
    // front_of_house::hosting::add_to_waitlist();
    hosting::add_to_waitlist(); // Path became available after the use statement at the beginning of the file

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}

// fn first_level_fn() {
//     println!("I'm the first level function");
// }

// mod first_level_mod {
//     fn secont_level_fn() {
//         super::first_level_fn();
//         fn third_level_fn2() {
//             super::super::first_level_fn();
//             fn fourth_level_fn() {
//                 super::super::super::first_level_fn();
//             }
//         }
//     }

//     mod second_level_mod {
//         fn third_level_fn() {
//             crate::first_level_fn();
//             super::super::first_level_fn();
//         }
//     }
// }
