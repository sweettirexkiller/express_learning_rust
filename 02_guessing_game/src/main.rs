use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 100);
    print!("Secret number is: {secret_number}\n");

    loop {
        println!("please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadowing
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small!".red()),
            Ordering::Greater => println!("{}","Too big!".red()),
            Ordering::Equal => {
                println!("{}","You win!".green());
                break;
            }
        }
    }
}
