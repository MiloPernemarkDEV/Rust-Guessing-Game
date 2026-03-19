use std::cmp::Ordering;
use std::io;
use rand::Rng;

#[derive(PartialEq)] // to use == with the enum
enum Difficulty {
    Hard,
    Medium,
    Easy,
    Fail,
}

fn checker(secret_number: u32, guess: u32) -> bool {
    match guess.cmp(&secret_number) {
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

fn ask_for_difficulty() -> Difficulty {
    println!("Choose difficulty:\nHard\nMedium\nEasy");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the line");

    match input.trim().to_lowercase().as_str() {
        "hard" => Difficulty::Hard,
        "medium" => Difficulty::Medium,
        "easy" => Difficulty::Easy,
        _ => Difficulty::Fail,
    }
}

fn generate_number(dif: Difficulty) -> u32 {
    match dif {
        Difficulty::Hard => rand::thread_rng().gen_range(1..=1000),
        Difficulty::Medium => rand::thread_rng().gen_range(1..=500),
        Difficulty::Easy => rand::thread_rng().gen_range(1..=100),
        Difficulty::Fail => 0, // Must handle all cases
    }
}

fn main() {
    let dif = ask_for_difficulty();

    // Use == for comparison
    if dif == Difficulty::Fail {
        println!("Invalid difficulty. Goodbye!");
        return;
    }
    
    let secret_number: u32 = generate_number(dif);

    println!("Guess the number!");
    let mut guess_count: i32 = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line!");

        // Changed type from 'num' to 'u32'
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            },
        };

        println!("You guessed: {guess}");
        
        // Pass value, not reference, to match checker's signature
        if checker(secret_number, guess) {
            println!("It took you {guess_count} number of guesses :)");
            break;
        }
        guess_count += 1;
    }
}
