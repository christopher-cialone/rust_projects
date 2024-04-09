use std::io;
use rand::Rng;
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

    if correct == guess {
        println!("You guessed the correct number!");
    }  else {
        println!("You have guessed incorrectly, my good sir")
    }

   
}