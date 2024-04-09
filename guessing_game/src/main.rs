use std::io;
use rand::Rng;
fn main() {
    let correct = rand::thread_rng().gen_range(1..=10);
    println!("correct: {correct}");
    println!("Guess the number from 1 to 10!");
    let mut num = String::new();

    // let mut guess = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");

    println!("You guessed: {}", num.trim());
}