use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("You started a guessing game");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Write your guess");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("Correct");
                break;
            }
            Ordering::Greater => println!("Too big"),
        }
    }
}