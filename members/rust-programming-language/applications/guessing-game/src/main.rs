use std::io;
use std::io::BufRead;

/// Follows [Programming a Guessing Game][guessing-game-tutorial].
/// [guessing-game-tutorial]: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
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
