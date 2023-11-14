use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");
        // mutable variable
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // reference to the mutable guess
            .expect("Failed to read line");

        // shadowing the mut guess String, parsing into u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }        
        }
    }
}