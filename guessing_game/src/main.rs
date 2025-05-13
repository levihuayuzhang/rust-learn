use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess the number! U fucking idot!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    println!("Just in put your fucking guess!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the fucking line!!!");

    let guess: u32 = guess.trim().parse().expect("Input integer");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small, dumb~"),
        Ordering::Greater => println!("Too big, lmao~"),
        Ordering::Equal => println!("It's right, u lucky buster!"),
    }
}
