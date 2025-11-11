
struct ImportantExcerpt<'a> {
    part: &'a str,
}


impl<'a> ImportantExcerpt<'a> {
    fn return_part(&'a self, announcement: &str) -> &'a str {
        println!("Announcement: {}", announcement);
        self.part
    }
}

fn main() {

    // dangling reference example
    // {
    //     let x = 5;
    //     r = &x;
    // }

    let x = 5;
    let r = &x;

    println!("r: {}", r);

    // let string1 = String::from("abcd");
    // let string2 = String::from("xyz");
    // let result = longest(string1.as_str(), string2.as_str());
    // println!("The longest string is {}", result);

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("Excerpt part: {}", i.part);

    let announcement = "This is an announcement.";
    println!("Return part: {}", i.return_part(announcement));

    let s : &'static str = "I have a static lifetime.";

}

// &i32 reference to an integer
// &'a i32 reference with lifetime 'a
// &'a mut i32 mutable reference with lifetime 'a

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}


// lifrtimes collision rules 
// 1. Each parameter that is a reference gets its own lifetime parameter.
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self, the lifetime of self is assigned to all output lifetime parameters.

fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where T: std::fmt::Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() { x } else { y }
}