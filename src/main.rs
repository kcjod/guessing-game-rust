use rand::Rng;
use std::{cmp, io};

fn main() {
    println!("-------- GUESS IT --------");
    println!("Guess a number: ");

    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input");

        let guess: u32 = match guess.trim().parse() {
            Ok(new) => new,
            Err(_) => {
                println!("Enter a valid number");
                continue;
            }
        };

        match guess.cmp(&secret) {
            cmp::Ordering::Equal => {
                println!("You guessed it!!!");
                break;
            },
            cmp::Ordering::Greater => println!("Too big"),
            cmp::Ordering::Less => println!("Too small"),
        }
    }
    println!("Thank youuu..❤");
}
