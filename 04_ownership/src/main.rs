fn main() {
    // ownership rules
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // {
    //     let s = String::from("hello"); // s is valid from this point forward
    //     // do stuff with s
    //     println!("{}", s);
    // } // this scope is over, and s is no longer valid

    // let x = 5;
    // let y = x; // copy, because i32 implements the Copy trait
    // println!("x = {}, y = {}", x, y);

    // let s1 = String::from("hello");
    // let s2 = s1; // move, s1 is no longer valid
    // // println!("{}", s1); // this would cause a compile-time error
    // println!("{}", s2);

    // let s = String::from("hello");
    // takes_ownership(s); // s's value moves into the function...
    //                     // ... and so is no longer valid here
    // // println!("{}", s); // this would cause a compile-time error

    // let x = 5;
    // makes_copy(x); // x would still be valid after this line
    // println!("{}", x);

    // let s1 = give_ownership(); // gives ownership to s1
    // println!("{}", s1);

    // let s2 = give_ownership(); // gives ownership to s2
    // let s3 = String::from("hello");
    // let s4 =  takes_and_gives_back(s3); // s3 is moved into the function and returned to s4
    // println!("{} {}",s2, s4);

    // let s1 = String::from("hello");
    // let len = calculate_length(&s1); // s1 is moved into the function
    // println!("The length of '{}' is {}.", s1, len);

    // let mut s2 = String::from("hello");
    // change(&mut s2);
    // println!("{}", s2);

    // let mut s = String::from("hello");

    // let r1 = & s; // no problem
    // let r2 = & s; // BIG PROBLEM
    // // let r3 = &mut s; // no problem

    // println!("{}, {}", r1, r2);

    // let r3 = & mut s;
    // println!("{}", r3);

    // let reference_to_nothing = dangle();
    // println!("{}", reference_to_nothing);

    // The rules of references
    // 1. At any given time, you can have either one mutable reference or any
    // number of immutable references.
    // 2. References must always be valid.

    // slice type
    let mut s = String::from("hello world");
    let s2 = "hello world";
    let hello = &s[..5];
    let world = &s[6..];
    println!("{} {}", hello, world);
    // let len = first_word(&s);
    // s.clear(); // this empties the String, making it equal to ""
    // println!("the length of the first word is: {}", len);

}

// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// }

// fn makes_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }

// fn give_ownership() -> String {
//     let some_string = String::from("yours");
//     some_string // move
// }

// fn takes_and_gives_back(a_string: String) -> String {
//     a_string // move
// }

// fn calculate_length(s: &String) -> usize {
//     let length = s.len();
//     length
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn dangle() -> &String {
    // This function will not compile because it returns a reference to a value that goes out of scope
    // let s = String::from("hello");
    // &s
// }


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}