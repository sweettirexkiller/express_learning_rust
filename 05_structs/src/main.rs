struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// associated function
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.pl"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // let name = user1.username;
    user1.username = String::from("anotherusername567");

    
    let user2 = build_user(String::from("asd@asd.pl"), String::from("asdasd"));

    let user3 = User {
        email: String::from("asd@asd.pl"),
        username: String::from("asdasdasd"),
        ..user2
    };


    // struct Color(i32, i32, i32);
    // struct Point(i32, i32, i32);

    // let width1 = 40;
    // let height1 = 50;


    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    print!("The area of the rectangle is {} square pixels.", area(&rect1));

    println!("rect1 is {:#?}", rect1);

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(3);
    println!("Square is {:#?}", square);

}


fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}