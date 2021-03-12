use std::io;
// Chapter 10 covers traits which is what Rng is.
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number!");
    println!("You have 10 tries to guess the number.");
    
    let secret_number = rand::thread_rng().gen_range(1, 100);
    let mut guesses = 10;

    while guesses > 0 {
        guesses -= 1;

        println!("Please input your guess!");

        let mut guess = String::new();

        // stores the inputted text into guess but then also returns a result
        // the return is an enum of type io::Result and it can be Ok or an error
        // we can then use unwrap() or expect("custom_message") to check for errors
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        // trim off the extra new line at the end and parse to signed 32 bit int
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => { // If you want to match on a specific error I suggest viewing: https://stackoverflow.com/a/49783839
                println!("That is not a number, dummy!");
                continue
            }
        };

        // For testing purposes, uncomment this
        // println!("You guessed {} and the secret was {}", guess, secret_number);

        // match will be covered more in ch 6 and 18
        // seems like it's similar to a switch statement but ensures you take into account
        // every scenario, so there's no choice in the matter, if there's a possible
        // enum value, you have to handle it someway
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"), // alt, put {} if you don't want to do anything
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
