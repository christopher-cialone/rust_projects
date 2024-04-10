use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    let mut how_many = String::new();
    println!("How many random numbers would you like to guess, Human-Being?");
    io::stdin()
        .read_line(&mut how_many)
        .expect("Failed to read line");

    let num_guesses: u8 = how_many.trim().parse().expect("Error reading input");

    let mut correct = Vec::new();

    for _ in 0..num_guesses {
        correct.push(rand::thread_rng().gen_range(1..=10));
    }

    println!("{correct:?}");


    let mut guesses_made = 0;

    println!("Guess a number from 1 to 10!");
    while guesses_made < num_guesses {
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

        match guess.cmp(&correct[guesses_made as usize]) {
            Ordering::Greater => println!("You guessed too high, no Bitcoin for you!"),
            Ordering::Less => println!("You have guessed too low, still no Bitcoin"),
            Ordering::Equal => {
                println!("You guessed correctly - we are owed a Bitcoin");
                guesses_made += 1;
                if guesses_made < num_guesses {
                    println!("Let's try the next one")
                }
            }
        };
    }
    println!("Thanks for playing the game - The correct answer were:");
    for item in correct {
        println!("{item}")
    }
}
