// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {
//             println!("add_to_waitlist");
//         }

//         fn seat_at_table() {
//             println!("seat_at_table");
//         }
//     }

//     mod serving {
//         fn take_order() {
//             println!("take_order");
//         }

//         fn serve_order() {
//             println!("serve_order");
//         }

//         fn take_payment() {
//             println!("take_payment");
//         }
//     }
// }

fn _serve_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        _seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                _seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn _fix_incorrect_order() {
        _cook_order();
        super::_serve_order();
    }

    fn _cook_order() {}
}

// binding with the whole crate in mind
// binding with the relative path (self/super)
mod front_of_house;

use crate::back_of_house::Breakfast;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // absolute path
    hosting::add_to_waitlist();

    // relative path
    hosting::add_to_waitlist();

    let mut meal = Breakfast::summer("rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // meal.seasonal_fruit = String::from("blueberries");
}
