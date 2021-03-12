fn main() {
    // You can't mark certain fields on a struct as mut
    // Either it is wholly mut or not
    let mut user = build_user(String::from("Jon"), String::from("jon@draplet"));
    println!("email is {}", user.email);

    if !user.email.ends_with(".com") {
        user.email.push_str(".com");
    }

    println!("email is now {}", user.email);

    user.pickle_count = 5;

    // Let's create a second user from the first
    // Similar to JS, slightly different, doesn't update
    // You set your specific fields first, remaining fields
    // are set from your original
    let user2 = User {
        name: String::from("Mr Pickles"),
        email: String::from("pickles@draplet.com"),
        ..user
    };

    println!("user2 email is {} and they have {} pickles", user2.email, user2.pickle_count);

    // we can even have smaller structs, called tuple structs
    // we only need to define the type name and field types in a tuple
    // struct
    // no need to name the fields!
    let blue_color = Color(1,3,250);
    let Color(r, g, b) = blue_color; // destructuing tuple struct
    let red = blue_color.0; // access one member in the struct
    println!("{}, {}, {}", r, g, b);

    // you can also have no members in your struct
    // this may be useful when we add traits in ch 10
    struct PerformSomeTask();

    let some_task = PerformSomeTask(); // woohoo
}

struct Color(i32, i32, i32);

fn build_user(name: String, email: String) -> User {
    // instantiate a struct
    User {
        name, // we can use shorthand just like JS
        email,
        active: true, // 
        pickle_count: 0
    }
}

// defining a struct
// Notice how we aren't storing references in our struct &
// this is on purpose, we can, but we need to learn lifetimes first
struct User {
    name: String,
    email: String,
    active: bool,
    pickle_count: u32
}