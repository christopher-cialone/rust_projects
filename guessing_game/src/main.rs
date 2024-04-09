use rand::Rng;
use std::io;
fn main() {
    let correct = rand::thread_rng().gen_range(1..=10);
    println!("correct: {correct}");
    println!("Guess the number from 1 to 10!");
    let mut guess = String::new();

    // let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Error with parse");

    let mut message = if correct < guess {
        String::from("You guessed too high, Cheech!")
    } else if correct > guess {
        String::from("You have guessed too low, Cretin")
    } else {
        String::from("You guessed the Perfectly")
    };

    println!("{message}");
}
