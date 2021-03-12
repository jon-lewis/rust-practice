fn main() {
    let s1 = "this is a primitive string and is not mutable";

    println!("primitive string: {}", s1);

    println!("{}", recurse(7));

    // Primitives and pointers live on the stack
    // primitive values and pointers/references are pushed onto the stack for a function call
    // all local variables are stored on the stock
    // the pointer can point to data allocated on the heap
    let mut s = String::from("hello");

    s.push(' ');
    s.push_str("pickles");

    println!("{}", s);

    // make another pointer to s's heap value
    // this makes s no longer valid
    // because at the end of the scope, s and s2 will need to be drop'd
    // so instead of freeing the same thing twice (bad)
    // when we make another pointer to the same heap space
    // we declare the old pointer as invalid
    // in order to fix the situation
    // the compiler will complain if you try to reference the old pointer
    // below the new one
    let mut s2 = s;

    s2.push('!');

    println!("s2: {}", s2);

    // This won't compile for the reason above!
    // println!("s: {}", s);

    // start a new scope with a new pointer to s2's data
    {
        let mut s3 = s2;
        s3.push_str(" How tasty are you?");
        println!("s3: {}", s3);
    }

    // s2 is still not valid here because s3 took ownership of the heap data
    // println!("s2: {}", s2);

    let mut s4 = String::from("Mr. Pickles");
    let mut s5 = s4.clone();
    
    // because s5 is a clone, we no longer have to worry about them both 
    // sharing the same heap space, they own their own spaces.
    s4.push_str(" Garage");
    s5.push_str(" Haus");

    println!("s4: {}, s5: {}", s4, s5);


    // What happens when we pass a string to a function
    // Well, because we make a copy to a function
    // if we pass the pointer then our pointer locally
    // is no longer valid

    let s6 = String::from("pickles");

    takes_ownership(s6);

    // s6 is no longer valid in this scope because takes_ownership took it
    // println!("s6: {}", s6);

    // we could have takes_ownership return it back to us when it's done
    let mut s7 = String::from("pickles");

    s7 = takes_ownership_v2(s7);

    println!("s7: {}", s7);

    // this is just a really weird way to do it but it can be done....
    // we could also return multiple values in a tuple kinda like javascript

    // but if we want to pass our heap variable to a function
    // and we know the function will not make changes tot he variable
    // well we can pass a reference and the compiler will make sure we 
    // don't change the variable in the function because it is borrowed
    // and it only has read access and the variable will continue to be
    // valid in this scope
    let s8 = String::from("here");
    let length = borrows(&s8);
    println!("s8: {} with a lenght of: {}", s8, length); // s8 is still valid here!

    // There is also dereferencing in ch 8 and 15
    // * is the deference symbol

    // strangely enough we can mutate a reference but we can only have one mutate reference in a scope at a time
    // this doesn't work:
    let s9 = String::from("can't change me");
    // cant_change_s9(&s9); // doesn't work
    println!("s9: {}", s9);

    let mut s10 = String::from("you can change me... but only 1 per scope can!");
    can_change_s10(&mut s10); // passing to a function does not count towards the 2 mutable references
    // can_change_s10(&s10); // doesn't work, you need to specify &mut mutable reference explicitly
    println!("s10: {}", s10);

    // this shouldn't work because of data races so only one mutable reference per scope is allowed
    let another_s10 = &mut s10;
    // let another2_s10 = &mut s10; // this doesn't work because we can only have 1 reference at a time per scope

    // This also doesn't work
    // {
    //     let another2_s10 = &mut s10; // this should work? Nope, because there would still be 2 in the this scope
    // }

    println!("another_s10: {}", another_s10);

    another_s10.push_str("pickles"); // this works only BEFORE the immutable reference

    // you also can't make changes to a mutable reference while there is immutable refs out there
    // because they don't expect the value to change.
    let immut_s10 = &s10;
    // another_s10.push_str("pickles"); // this works only BEFORE the immutable reference
    // println!("s10: {}", another_s10); // we reference our mutable reference after the immutable ref... this doesn't work

    let mut s11 = String::from("phew");
    let r1_s11 = &s11;
    let r2_s11 = &s11;
    println!("s11, r1: {}, r2: {}", r1_s11, r2_s11);
    let mr1_s11 = &mut s11;
    println!("s11, mr1: {}", mr1_s11);
    // println!("s11, r1: {}, r2: {}", r1_s11, r2_s11); // can't use the references after the mutable reference!!!! because the value might have changed!

    // NOTE: a reference's scope starts when it is declared and ends the last time it was referenced. This is slightly
    // different from a variables/ownership scope.

    // Let's try to make a dangling reference which could get freed before our pointer is destroyed
    // rust won't let you call the following function
    // let reference_to_nothing = dangle();
}

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s // s is going out of scope and will be deallocated

//     // in order to solve this, you must return s directly
//     // s (don't forget to change the return type to String)
// }

fn can_change_s10(s: &mut String) {
    s.push_str("... yeah it works");
}

// fn cant_change_s9(s: &String) {
//     s.push_str("doesn't work :(");
// }

// does not take ownership of the given parameter, only recieves a reference to the object instead
fn borrows(s: &String) -> usize {
    let mut cloned = s.clone(); // we can still clone the reference but we can't change the original
    cloned.push_str(" I am!");
    println!("cloned: {}", cloned);
    s.len()
}

// if we didn't want s to be mutable
// s: String is fine too
fn takes_ownership_v2(mut s: String) -> String {
    s.push_str(" haus");
    println!("take ownership v2: {}", s);
    s // return s
}

fn takes_ownership(s: String) {
    println!("take ownership: {}", s);
}

fn recurse(x: i32) -> i32 {
    if x * 3 > 30 || x * 3 < -30 {
        return x
    }

    recurse(x * 3)
}
