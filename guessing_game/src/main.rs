use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    let correct = rand::thread_rng().gen_range(1..=10);
    println!("correct: {correct}");
    println!("Guess the number from 1 to 10!");
    // let mut guess = String::new();

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("Error with parse, try again. {e}");
                continue;
            }
        };

        match guess.cmp(&correct) {
            Ordering::Greater => println!("You guessed too high, Cheech!"),
            Ordering::Less => println!("You have guessed too low, Cretin"),
            Ordering::Equal => { 
                println!("You guessed the Perfectly"); 
                break;
            },
        };
    }
}
