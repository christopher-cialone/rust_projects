use std::io; 
fn main() {
    println!("Guess the number");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin() // allows us to handle user input
        .read_line(&mut guess)
        .expect("failed to read line");

    println!("Look here, Chicken, You guessed: {guess}");
}
