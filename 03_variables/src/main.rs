fn main() {
    // let x = 5;
    // println!("The value of x is: {}", x);
    // // first x is shadowed by the second x!!
    // let x = "six";
    // println!("The value of x is: {}", x);

    // const SUBSCRIBER_COUNT: u32 = 100000;

    // scalar types vs compound types

    // intigers
    let a = 98_222;
    let b: u8 = 255;
    let c: i8 = -128;
    // binaruy
    let d: u8 = 0b1111_0000;
    // octal
    let e: u8 = 0o77;
    // hexadecimal
    let f: u8 = 0xFF;
    // byte

    let f: isize = 64;
    // floiating point numbers
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
                      // booleans
    let t = true;
    let f: bool = false;
    // characters
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    // compound types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (channel, sub_count, active) = tup;
    let sub_count2 = tup.1;

    // loops can return values
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // different kinds of loops
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // iterating through a collection with for
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    // functions, there is no return keyword, just expressions
    fn five() -> i32 {
        5
    }

    let x = five();
    println!("The value of x is: {}", x);
}
