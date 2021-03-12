fn main() {
    let male = SexKind::Male;
    println!("sex 1: {:?}", male);

    let male = SexStatsKind::Male(String::from("penis"), 330, 401, 89);
    println!("male w/ stats: {:?}", male);

    // call a method on the enum
    let message = Message::Quit;
    message.do_something();


    // The std lib comes with the Option enum
    // so we can take care of traditional null values
    // enum Option<T> {
    //   Some(T),
    //   None,
    // }

    // This is what the 
    let some_value = Option::Some("74");
    let some_value = Some("74");
    let no_value: Option<u32> = None;

    // Now we can use the match expression to take care
    // of an Option enum, making sure to take into account
    // a null case

    // match expression
    println!("value of a dime is {}", value_in_cents(Coin::Dime));
    println!("value of an Alabama quarter is {}", value_in_cents(Coin::Quarter(UsState::Alabama)));

    // let's look at using option in a function definition
    println!("value of 5 + 1 is {:?}", add_one(Some(5)));
    println!("value of None + 1 is {:?}", add_one(None));

    // if we have multiple arms that we don't need to cover we can use _
    let some_u8_value = 5u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    // instead of writing a match expression
    // you can use if let instead
    let coin = Coin::Dime;

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else if let Coin::Dime = coin {
        println!("picllek");
    } else {
        println!("No quarter...");
    }
}

fn add_one(num: Option<i8>) -> Option<i8> {
    match num {
        None => None,
        Some(i) => Some(i + 1)
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            // do some stuff here
            1 // return value
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("the quarter is from {:?}", state);
            25
        },
    }
}

#[derive(Debug)]
enum SexKind {
    Male,
    Female
}

// We can associate data with the enums
// They can have different data
#[derive(Debug)]
enum SexStatsKind {
    Male(String, u32, u32, u32), // You can even store a struct here (it's just a tuple of data)
    Female(String, u32)
}

// another example of structs
// Grouping possibilities and associated data is powerful when we pass data to functions and back.
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // this is an anonymous struct
    Write(String),
    ChangeColor(i32, i32, i32),
}

// You can even define methods on enums, just like structs
impl Message {
    fn do_something(&self) {
        // do something here...
    }
}