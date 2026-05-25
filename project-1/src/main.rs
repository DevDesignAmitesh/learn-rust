use std::{cmp::Ordering, io};
use rand::Rng;


fn main() {
    loop {
        println!("Guess the number");

        println!("Please enter your guessed number");

        // generating the secret number
        let secret_number = rand::thread_rng().gen_range(1..=100);
        
        println!("This is the secret number: {secret_number}");

        let mut guess = String::new();

        // getting the guess from the users
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
            
        
        // handling errors
        let my_int_guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("error while parsing {err}");
                continue;
            },
        };
    
        // the rust way
        match my_int_guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You win");
                break;
            },
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too small")
        }
        
        
        // the basic way 
        // converting to int so that get compared easily
        // if my_int_guess == secret_number {
            // println!("you guessed the right word: {guess}");
        // } else {
            // println!("you guessed the wrong word: {guess}");
        // }
    }
        
}
