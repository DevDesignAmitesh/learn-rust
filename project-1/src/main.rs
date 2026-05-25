use std::io;

use rand::Rng;


fn main() {
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
        
    // converting to int so that get compared easily
    let my_int_guess: i32 = guess
        .trim()
        .parse()
        .expect("unable to convert it to number");
        
    if my_int_guess == secret_number {
        println!("you guessed the right word: {guess}");
    } else {
        println!("you guessed the wrong word: {guess}");
    }
        
}
