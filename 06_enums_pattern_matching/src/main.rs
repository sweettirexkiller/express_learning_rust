enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn some_func() {
        println!("Hello from some_func");
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

// fn route(ip_kind: IpAddrKind) {

// }

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    California,
    Colorado,
    Connecticut,
    Delaware,
    Florida,
    Georgia,
    Hawaii,
}


fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

/*.
MAIN
*/
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let localhost = IpAddrKind::V4(127, 0, 0, 1);

    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i8> = None;

    let x = 5;
    // let y = Some(5);
    let y: Option<i8> = None;

    let sum = x + y.unwrap_or(0);

    println!("The sum is: {}", sum);

    let coin = Coin::Dime;
    let value = value_in_cents(coin);
    println!("The value of the coin is: {} cents", value);

    let coin2 = Coin::Quarter(UsState::California);
    let value2 = value_in_cents(coin2);
    println!("The value of the coin is: {} cents", value2);


    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("six is {:?}, none is {:?}", six, none);

    let some_value = Sone(3);
    match some_value {
        Some(3) => println!("three"),
        _ => (),    
    }

    // IF LET SYNTAX is strnge
    // if let Some(3) = some_value {
    //     println!("three");
    // }
}
