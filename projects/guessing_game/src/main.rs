// bring in the io libray in to scope to use inputs and outputs
// io library comes from the standard library
// the prelude is a standard library automatically included in rust files
// if something is not in the prelude then you have to bring it in using use
use std::io;

// main function is entry point into the program
// fn syntac declares a new function
// parentheses are empty so no parameters
// brackets contain the body of the function
fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // varible mul for storing user input
    // use let statement to create the variable
    // in rust vars are immutible by default so use let to make sure this variable is mutable
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}