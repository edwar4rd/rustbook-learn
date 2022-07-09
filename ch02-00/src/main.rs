use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!\n");

    let secret_number = rand::thread_rng().gen_range(1..=10);
    // println!("The secret number is: {secret_number}");

    loop {  
        println!("Please input your guess: ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess = guess.trim();
        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("*throw up");
                println!("*spits out garbage");
                println!("...");
                continue;
            },
        };
        println!("You guessed: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => {
                println!("correct!!!");
                break;
            },
        }
    }
}
