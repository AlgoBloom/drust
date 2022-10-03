// bring in the io libray in to scope to use inputs and outputs
// io library comes from the standard library
// the prelude is a standard library automatically included in rust files
// if something is not in the prelude then you have to bring it in using use
use std::io;
// importing the range methods from the random library
use rand::Rng;

// main function is entry point into the program
// fn syntac declares a new function
// parentheses are empty so no parameters
// brackets contain the body of the function
fn main() {
    println!("Guess the number!");

    // defining a secret number to make the game more fun
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // printing the secret number
    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    // varible guess for storing user input
    // use let statement to create the variable
    // in rust vars are immutible by default so use let to make sure this variable is mutable
    // mut adds a type of mutable to this variable making it mutable
    // this variable returns a new string
    // allows multiple parts of your code to access one piece of data without needing to copy that data into memory multiple times
    let mut guess = String::new();

    // calling stdin function allows us to handle user input
    io::stdin()
        // calls the read line method
        // passing the guess variable into the read line method
        // this allows us to tell guess what to store
        // takes user input and appends it to a string
        // & indicates that this argument is a reference
        // must continue using mut to clearly define the variable as mutable
        // read line returns a Result value of type enum, or enumeration, which is a type that can be in one of multiple possible states, we call each possible state a variant
        // purpose of Result types is to encode error handling information
        .read_line(&mut guess)
        // the Result's variants are Ok and Err
        // Ok variant indicates the operation was successful and inside Ok is the successfully generated value
        // Err variant means the operation failed and Err contains information about how or why the operation failed
        // values or the Result type, like values of any type, have methods defined on them
        // an instance of Result has an expect method that you can call
        // we use except becuase we want to crash the program when a problem occurs
        .expect("Failed to read line");

    // this line prints the string that now contains the user's input
    println!("You guessed: {guess}");
    // trivial change
}