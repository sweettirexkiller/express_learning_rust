
pub struct NewArticle {
    pub headline: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}


pub trait Summary {
    fn summarize(&self) -> String;
    fn summarize_author(&self) -> String {
        String::from("Anonymous")
    }
}

// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary + std::fmt::Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// best syntax
fn some_func<T,U>(t: &T, u: &U) -> i32
where
    T: std::fmt::Display + Clone,
    U: std::fmt::Debug + Clone,
{
    1
}


fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}


// // not allowed 
// fn returns_summarizable2(switch: bool) -> impl Summary {
//     if switch {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     } else {
//         NewArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//         }}}


struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// implementing a trait for all types that implement another trait
impl<T:Display> ToString for T {
    fn to_string(&self) -> String {
        // implementation goes here
    }
}


fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize_author());

    let article = NewArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best \
                   hockey team in the NHL."),
    };

    println!("New article available! {}", article.summarize());

    notify(&tweet);

    println!("1 new tweet: {}", returns_summarizable().summarize());

    let pair = Pair::new(3, 5);
    pair.cmp_display();

}
