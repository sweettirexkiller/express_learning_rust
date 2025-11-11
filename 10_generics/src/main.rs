fn main() {
    // lartest on generics
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    let mixed_point = Point { x: 5, y: 4.0 };

    println!("integer_point.x = {}", integer_point.x());
    println!("float_point.distance_from_origin = {}", float_point.distance_from_origin());

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let some_number = Option::Some(5);
    let some_string = Option::Some("a string");
    let absent_number = Option::<i32>::None;



}

// generic find largest function
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];  
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// generic in structs
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

enum Option<T> {
    Some(T),
    None,
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}