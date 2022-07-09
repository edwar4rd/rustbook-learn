use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!\n");

    let secret_number = rand::thread_rng().gen_range(1..=10);
    let mut guess = String::new();

    println!("Please input your guess: ");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess = guess.trim();
    println!("You guessed: {guess}");
    let guess: u32 = guess.parse().expect("Unable to parse to number");

    println!("The secret number is: {secret_number}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small!"),
        Ordering::Greater => println!("too big!"),
        Ordering::Equal => println!("correct!!!"),
    }
}
