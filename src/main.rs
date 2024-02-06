use std::io;
// This calls io from the standard library (std)
fn main() {
    println!("Guess the secret number");
    println!("Hint: The number is between 1 and 100 (inclusive)");
    println!("===============================");
    
    // `mut` because I want the user to be able to change their guess.
    // `new` is an associated function of the String type
    let mut guess = String::new();
   
    // This calls stdin from the io module  
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read guess");
    println!("You guessed: {}", guess);
    
    // NOTE: How readline works

    // The full job of read_line,
    // is to take whatever the user types into standard input,
    // and append that into a string (without overwriting its contents),
    // so we therefore pass that string as an argument.
    // The string argument needs to be mutable, 
    // so the method can change the string’s content.

}
