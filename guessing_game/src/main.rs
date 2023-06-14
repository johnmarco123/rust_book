use std::{io, cmp::Ordering};

use rand::Rng;

fn get_guess_from_user() -> String {
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}");
    return guess;
}

fn main() {
    println!("Guess the number!");

    loop{
        let secret_number = rand::thread_rng().gen_range(1..=100);
        let guess: String = get_guess_from_user();
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
