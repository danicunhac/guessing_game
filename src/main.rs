use std::io;

fn main() {
    println!("Hello, world!");

    println!("Please input your guess");

    // Use "mut" to make a variable mutable
    // > Variables are immutable by default
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}")
}
