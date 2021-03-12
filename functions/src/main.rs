fn main() {
    println!("Hello, world!");

    calculate_something(5, 6);

    println!("calculated: {}", calculate_something_return(7, 8));

    let x = calculate_something_return(7, 8);

    if x < 10 {
        println!("x is less than {}", 10);
    } else {
        println!("x is greater than 10");
    }

    let to_be = "it is to be";

    // interesting, no compiler check if the same variable is compared against itself
    let to_be = if to_be != to_be { "it is" }  else { "it is not" };

    println!("{} to be.", to_be);

    // basic loop
    // loop {
    //     println!("loop forever! CTRL + c to quit");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            // this is the only way to break out of a loop since there are no return statements
            break counter * 2;
        }
    };

    println!("The counter is now {}", result);


    // woohoo while loops
    let mut number = 5;

    while number > 0 {
        number -= 1;
    }

    println!("LIFTOFF!!!");


    // woohoo for loops
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    let a = [1,3,8,190];

    for e in a.iter() {
        println!("for loop {}", e);
    }

    // a range from 1 to 4 (exclusive) and then call reverse fn
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

     // a range from 1 to 4 (inclusive) and then call reverse fn
     for number in (1..=4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    //ranges also work with char values but that's it
    let alphabet = 'a'..='z';

    for letter in alphabet {
        println!("alphabet: {}", letter);
    }
}

fn calculate_something(x: i32, y: i32) {
    println!("Calculating... {} + {} = {}", x, y, x + y);
}

fn calculate_something_return(x: i32, y: i32) -> i32 {
    // not that semicolons denote statements and statements do not return a value
    // DO NOT PUT A SEMICOLON WHEN YOU WANT TO RETURN FROM AN EXPRESSION!
    // {} create a new scope / expression
    x + y
}
