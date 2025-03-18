use std::io;

fn main() {
    println!("Guess the number! U fucking idot!");
    println!("Please in put your fucking guess!");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the fucking line!!!");

    println!("You guessed: {}", guess);

}