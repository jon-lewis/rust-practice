use std::fs::File;
use std::fs::Error;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() {
    let f = File::open("hello.txt");

    match f {
        Ok(file) => println!("We got the file {:?}", file),
        Err(error) => println!("We got an error {}", error)
    };


    let f = File::open("hello.txt");

    // We could match on multiple errors
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            },
        },
    };

    // Cleaner to do it this way, so we don't have a bunch of match expressions.
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            }) // unwraps the new created file or panics
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });


    // unwrap the result or panic, not recommended for most use cases
    // let f = File::open("doesnotexist.txt").unwrap();

    // unwrap or provide a default for the type returned
    // let f = File::open("doesnotexist.txt").unwrap_or(default: T);

    // fn default_fn(error: Error) -> File {
    //     File::create("pickles.txt").unwrap()
    // }

    // provide a function that accepts the error and provides a default
    // let f = File::open("doesnotexist.txt").unwrap_or_else(default_fn);

    // panic if there is an Ok value, return the err value
    let error = File::open("doesnotexist.txt").unwrap_err();


    let good_year_from_input = "1909";
    let bad_year_from_input = "190blarg";
    let good_year: u32 = good_year_from_input.parse().unwrap_or_default();
    let bad_year: u32 = bad_year_from_input.parse().unwrap_or_default();

    if bad_year > 0 {
        println!("");
    }
}


fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}