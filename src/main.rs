use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Please tell me your guess: ");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the guess.");

    let guess: u32 = guess.trim().parse().expect("Please type a number");

    println!("You guessed: {}", guess);
    println!("The secret generated number was: {secret_number}");
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("The number was to low."),
        Ordering::Greater => println!("The number was to big."),
        Ordering::Equal => println!("Good Job. You won :)"),
    }
}
