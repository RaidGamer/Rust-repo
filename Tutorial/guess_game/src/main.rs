use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=100); //första kommentar :D

    println!("Guessing Game!");
    println!("The secret number is {secret_number}.");

    loop {

    println!("What is your guess?: ");

    let mut guess: String = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line!");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Wrong value!");
            continue;
        }
    };

    println!("You guessed: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Greater => println!("Too big!"),
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => {
            println!("You win!!!!!");
            break;
            }

        }
    
    }

}