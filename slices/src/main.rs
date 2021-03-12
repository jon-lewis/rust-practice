fn main() {
    let mut words = String::from("hello there");
    println!("{}", first_word(&words));

    // slices
    let hello = &words[0..5]; // rust will panic at runtime if this is out of bounds
    let world = &words[6..11]; // these are immutable references to the words variable and thus guarantees that it won't be mutated

    let len = words.len();

    let slice = &words[3..len];
    let slice = &words[3..]; // same as 3..len
    let slice = &words[..3];

    // you can't mutate the string here...

    println!("slices: '{}' and '{}'", hello, world);

    words.push_str("pickles");

    println!("slices");

    println!("using a string literal which is an str: {}", first_word("pickles sold here"));


    println!("number slices");

    let a = [1,2,5,8];

    // we'll talk more on the various collection/slice types in ch 8
    let slice = &a[1..3]; // this slice is type &[i32]

    println!("1..3, first element is: {}, len is: {}", slice[0], slice.len());
}

// fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    // we'll talk more on iter and enumerate in ch 13.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i]; // slice
        }
    }

    &s[..] // slice of the whole string
    // s.len() // using length here is error prone because what if we change the original string?
    // then it'll be out of sync
    // what if we could use a slice!
}