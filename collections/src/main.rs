use std::collections::hash_map::HashMap;

fn main() {
    let mut v = vec![1,2,3];

    println!("2nd element in vector/heap array: {}", &v[1]);

    v[1] = 7;

    println!("2nd element in vector/heap array: {}", &v[1]);

    // println!("7th element does not exist and will panic: {}", &v[7]);
    println!("7th element in vector/heap array: {:?}", v.get(7)); // returns an Option enum instead of panic

    match v.get(7) {
        Some(x) => println!("found a value: {}", x),
        None => println!("Couldn't find anything")
    }

    v.push(73); // add an array element

    println!("array now looks like: {:?}", v);

    for i in &v {
        println!("{}", i);
    }

    for i in &mut v {
        *i += 1;
        println!("{}", i);
    }

    let mut pickles = vec![Pickle{tasty:3}];

    for p in &mut pickles {
        p.tasty = 7; // equivalent to the next line
        (*p).tasty = 7; // don't need to dereference this, rust will do it automatically
    }

    println!("pickles array now looks like: {:#?}", pickles);


    let mut ints: Vec<i32> = Vec::new();

    ints.push(1);
    ints.push(10);

    // Find the average of the ints
    let mut total = 0;
    for i in &ints {
        total += i;
    }

    let length = ints.len();
    match length {
        0 => println!("Average is 0"),
        _ => println!("Average of ints: {}", total as usize / length)
    }

    // Find the mean of the ints
    
    // First, sort the array
    ints.push(-3);
    ints.push(74);

    ints.sort();

    let length = ints.len();
    
    println!("the median is {:?} in {:?}", ints.get(length / 2), ints);

    // Find the mode of the array (the value that appears the most)

    ints.push(10);
    ints.push(11);
    ints.push(11);
    ints.push(11);
    
    let mut map = HashMap::new();
    
    for i in &ints {
        let entry = map.entry(i).or_insert(0);
        *entry += 1; // keep a count of how many times this thing shows up.
    }

    println!("Here's the map: {:#?}", map);

    let mut max_key: i32 = 0;
    let mut max_count: u32 = 0;

    for (k, v) in &map {
        if *v > max_count {
            max_key = **k;
            max_count = *v;
        }
    }

    println!("the mode is {:?}", max_key);


    // Convert strings to pig latin. 
    // The first consonant of each word is moved to the end of the word and “ay” is added,
    // so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to 
    // the end instead (“apple” becomes “apple-hay”). 
    // Keep in mind the details about UTF-8 encoding!

    let original_string = String::from("first");

    println!("converted string: {}", pig_latin(&original_string));
    println!("converted string: {}", pig_latin(&String::from("orange")));
}

fn pig_latin(s: &String) -> String {
    let vowels = ["a","e","i","o","u"];
    let first_letter = &s[0..1];

    if vowels.contains(&first_letter) {
        return s.clone() + "-hay";
    } 
    
    String::from(&s[1..]) + "-" + first_letter + "ay"
}

#[derive(Debug)] // allows print
struct Pickle {
    tasty: u32
}