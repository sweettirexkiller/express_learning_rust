    use std::collections::HashMap;



fn main() {



    println!("Hello, world!");

    let mut vec = vec![1, 2, 3, 4, 5];
    let third = vec[2];
    // v.push(6);
    // println!("The third element is {}", third);

    match vec.get(1) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    for i in &mut vec {
        *i += 50;
    }

    for i in &vec {
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    match &row[1] {
        SpreadsheetCell::Int(i) => println!("Int: {}", i),
        _ => println!("Not an integer"),
    }

    // strings
    // strings are UTF-8 encoded bytes
    let mut s = String::new();  

    // hash maps
    let blue = String::from("Blue");
    let yellow = String::from("Yellow");

    let mut scores = HashMap::new();
    scores.insert(blue, 10);
    scores.insert(yellow, 50);
    
    scores.entry(String::from("Green")).or_insert(30);
    scores.entry(String::from("Blue")).or_insert(25);


    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match score {
        Some(score) => println!("Score: {}", score),
        None => println!("No score"),
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}
