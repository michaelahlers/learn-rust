use std::io;
use std::io::BufRead;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess:");

    io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .map(|guess| {
            println!("You guessed: \"{guess}\".");
        })
        .expect("Failed to read line.");
}
