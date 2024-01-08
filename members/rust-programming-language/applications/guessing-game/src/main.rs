use std::io::stdin;
use std::io::Result;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess: String = String::new();

    let result: Result<usize> = stdin().read_line(&mut guess);
    result.expect("Failed to read line.");

    println!("You guessed: \"{guess}\".");
}
