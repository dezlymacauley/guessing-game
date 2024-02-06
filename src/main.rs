use std::io;
use std::io::Write; // This is needed to use the flush method
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    // NOTE: Let the player know how the game works 
    
    println!("Guess the secret number");
    println!("Hint: The number is between 1 and 50 (inclusive)");
    println!("===============================");
    
    // NOTE: Generate the secret number

    let secret_number = rand::thread_rng().gen_range(1..=60);
   
    // NOTE: Create a loop that will allow the player to keep guessing

    loop {
        
        // NOTE: Allow the user to enter their guess then store it

        print!("Enter your guess: ");
        io::stdout().flush().expect("Failed to flush");
        // This wil make sure the prompt is displayed like this:
        // Enter your guess: 18 

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // NOTE: Check if the player's guess was correct

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("The guess was too low."),
            Ordering::Greater => println!("Your guess was too high."),
            Ordering::Equal=> {
            println!("Your guess was correct!");
            println!("The secret number was: {}", secret_number);
            break;
            }
        }
    }
    
}
