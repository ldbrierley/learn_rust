extern crate rand;
use rand::Rng;
use std::io;
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret numberis:{}", secret_number);
    println!("please input guess");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
    println!("You guessed: {}", guess);
}
