use crate::front_of_house::hosting; // as pickle; 

pub fn make_food(item: &String) {
    println!("Prepping...");
    println!("Cooking...");
    println!("{} is ready!", item);

    // this should really go to a job queue and serve_food reads from the queue
    // separation of concerns and event driven programming instead of making a hard link here
    hosting::serve_food(item);
    // pickle::serve_food(item);
}