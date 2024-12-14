use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
        println!("You guessed {guess}");
    }
}
