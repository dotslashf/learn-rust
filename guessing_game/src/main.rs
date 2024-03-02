use std::{cmp::Ordering, io};

use rand::Rng;
// use rand::

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut guess_counter: u32 = 0;

    loop {
        let mut guess = String::new();
        println!("Enter your answer:");

        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                guess_counter += 1;
                println!("Too small! You guessed {} times", guess_counter);
            }
            Ordering::Equal => {
                println!("You guess correctly in {} times!", guess_counter);
                break;
            }
            Ordering::Greater => {
                guess_counter += 1;
                println!("Too big! You guessed {} times", guess_counter);
            }
        }
    }
}
