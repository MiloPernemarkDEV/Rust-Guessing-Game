use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn checker(secret_number: &u32, guess: &u32) -> bool {
    match guess.cmp(secret_number) {
        Ordering::Less => { 
            println!("Too small!");
            false
        }
        Ordering::Greater => {
            println!("Too big!");
            false
        }
        Ordering::Equal => { 
            println!("You win!");
            true
        }
    }
}

fn main() {
    println!("Guess the number!");

    // implicitly immutable
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    // println!("The secret number is: {secret_number}");
    let mut guess_count: i32 = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("You guessed: {guess}");
        
        if checker(&secret_number, &guess) == true {
            println!("It took you {guess_count} number of guesses :)");
            break;
        }
        guess_count += 1;
    }
}
