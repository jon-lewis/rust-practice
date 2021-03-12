const MAX_PICKLES: u32 = 100_000; // same as 100000 but easier to read when it's broken up w/ _'s

fn main() {
    // constants can't be set from a function call or any other means
    // constants are constant for the life of the program and can be defined in any scope
    const MAX_PICKLES_NO_UNDERSCORE: u32 = 100000;

    println!(
        "The value of our constants is {} == {}",
        MAX_PICKLES, MAX_PICKLES_NO_UNDERSCORE
    );

    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6; // x must be declared as mutable using 'mut'
    println!("The value of x is {}", x);

    // Ok, can't change the type after assigned... cool
    // x = "hey";
    // println!("The value of x is {}", x);
    // x = 0.5; // can't change int to float or float to int... cool
    // println!("The value of x is {}", x);

    // WOW, you can redeclare a variable... strange
    // ok, I get it, this is called shadowing: https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing
    // let x = 6;
    // println!("The value of x is {}", x);

    // Here's a good example of shadowing and changing the type of the variable we are shadowing
    // shadowing allows us to create new variables with different types without having to name
    // things differently like num_spaces, str_spaces
    let spaces = "   ";
    println!("spaces is '{}'", spaces);

    let spaces = spaces.len();
    println!("# of spaces is {}", spaces);

    // data types

    // we need to tell the compile what kind of number we expect from parsing since
    // it doesn't know what kind of number it will be.
    // Unsigned 32 bit int
    // TODO I'm not sure what this expect function is...
    let answer_to_the_universe: u32 = "42".parse().expect("This is not a number!");
    println!("universe is {}", answer_to_the_universe);

    let x: i32 = 2 - 4;
    // this would produce an overflow error because negative numbers must be signed
    // let x: u32 = 2 - 4;

    // If you would like to perform overflow on purpose: https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow

    println!("signed number {}", x);

    // Addition must be with the same types
    // println!("float + int is {}", 64 + 3.3);

    let heart_face: char = '🥰';

    println!("I {1} {0}", "Rust!", heart_face);

    // tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("{} is the same as {}, {} {}", tup.0, x, y, z);

    // arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5]; // five elements that will be set to 3 e.g. [3,3,3,3,3]

    println!("a[0] is {}", a[0]);
}
