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

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

fn deliver_order() {}

mod customer {
    pub use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        // Absolute path
        crate::front_of_house::hosting::add_to_waitlist();

        // Relative path
        super::front_of_house::hosting::add_to_waitlist();

        // With 'use'
        hosting::add_to_waitlist();

        // Order a breakfast in the summer with Rye toast.
        let mut meal = super::back_of_house::Breakfast::summer("Rye");
        // Change our mind about what bread we'd like.
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        // The next line won't compile if we uncomment itl we're not
        // allowed to see or modify the seasonal fruit that comes
        // with the meal.
        // meal.seasonal_fruit = String::from("blueberries");

        let order1 = super::back_of_house::Appetizer::Soup;
        let order2 = super::back_of_house::Appetizer::Salad;
    }
}
