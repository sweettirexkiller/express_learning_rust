fn main() {
    // ownership rules
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    {
        let s = String::from("hello"); // s is valid from this point forward
        // do stuff with s
        println!("{}", s);
    } // this scope is over, and s is no longer valid

    let x = 5;
    let y = x; // copy, because i32 implements the Copy trait
    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1; // move, s1 is no longer valid
    // println!("{}", s1); // this would cause a compile-time error
    println!("{}", s2);

    let s = String::from("hello");
    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here
    // println!("{}", s); // this would cause a compile-time error

    let x = 5;
    makes_copy(x); // x would still be valid after this line
    println!("{}", x);

    let s1 = give_ownership(); // gives ownership to s1
    println!("{}", s1);

    let s2 = give_ownership(); // gives ownership to s2
    let s3 = String::from("hello");
    let s4 =  takes_and_gives_back(s3); // s3 is moved into the function and returned to s4
    println!("{} {}",s2, s4);
}


fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn give_ownership() -> String {
    let some_string = String::from("yours");
    some_string // move
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // move
}