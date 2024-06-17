// Import the standard library's input/output module.
use {rand::Rng, std::cmp::Ordering, std::io};

// The main function is the entry point of the program.
fn main() {
    // Use the println! macro to prompt the user for input.
    /*2 - GENERATING A SECRET NUMBER */
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The Secret Number is {secret_number}");

    println!("Please input your guess.");

    /*1 - GET THE INPUT FROM THE USER */
    loop {
        println!("Please input your new guess.");
        // Declare a mutable variable named "guess" and initialize it with an empty string to store the user's input.
        let mut guess = String::new(); // String::new() is a function from the standard library that returns a new instance of a string.

        // Read a line of input from the user and store it in the "guess" variable.
        // The "io::stdin()" function returns an instance of "std::io::Stdin" that allows reading from the standard input.
        // The ".read_line()" method reads a line from the standard input into the "guess" variable.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line."); // The '&' symbol indicates that the argument is a reference, allowing multiple parts of the code to access the same piece of data without copying it into memory multiple times.
                                             // By default, references are immutable, but here we make it mutable using '&mut'.
                                             //shadow the previous value eith a new one.

        // Similar to the try:..except as e : ... in Python. Catch an possible error and decide what to do next if it occurs.
        let guess: u32 = {
            match guess.trim().parse() {
                Ok(value) => value,
                Err(_) => {
                    println!("Wrong value, please enter an integer...");
                    continue;
                }
            }
        };

        println!("Here is the guessing value: {guess}");

        /*3 - COMPARING GUESS AND SECRET NUMBER */

        match secret_number.cmp(&guess) {
            Ordering::Equal => {
                println!("You Win !");
                break;
            }
            Ordering::Greater => println!("Too Big !"),
            Ordering::Less => println!("Too Small !"),
        }
    }
}
