use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Secret number is {}", secret_number);

    let mut guess = String::new();

    println!("Please input your guess number:");
    io::stdin()
        .read_line(&mut guess)
        .expect("fail to read");

    println!("Your guessed {}", guess);
}
