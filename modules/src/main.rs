mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(name: &String) {
            println!("Added {} to waitlist", name);
        }

        pub fn serve_food(item: &String) {
            println!("Serving {} to table...", item);
        }
    }
}

// We could define a mod here but as it grows
// we will want the functionality to be in different files

// mod back_of_house {
//     pub mod kitchen_crew {
//         pub fn make_food(item: &String) {
//             println!("Prepping...");
//             println!("Cooking...");
//             println!("{} is ready!", item);

//             // this should really go to a job queue and serve_food reads from the queue
//             // separation of concerns and event driven programming instead of making a hard link here
//             crate::front_of_house::hosting::serve_food(item);
//         }
//     }
// }

// Defining back_of_house like this, we are now taking advantage of importing and then
// auto publicizing the module in one statement (make sure you have a semicolon at the end!)
mod back_of_house;

fn main() {
    front_of_house::hosting::add_to_waitlist(&String::from("Jon party of 1"));
    back_of_house::kitchen_crew::make_food(&String::from("Fried Pickles"));
}
