use std::io;    //Allows access to the standard Input/Output
use rand::Rng;  //Allows access to the random library and the RNG operation
use std::cmp::Ordering;     //Allows access to the comparison Ordering operation

fn main() {     //This is the start of the main function
    println!("Guess the number!");      //"Print Line" macro

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is: {secret_number}"); //Used for testing

    loop {  //Adds a looping function to the program to keep it active after guessing

    println!("Please input your guess:");   //"Print Line" macro

    let mut guess = String::new();  //Creates a mutable variable named "guess"

    io::stdin()     //This calls the standard input option to handle user input
        .read_line(&mut guess)  //This reads the input and assigns it to "guess"
        .expect("Failed to read line"); //Handles a possible error that could pop up

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("You guessed: {guess}");   //"Print Line" macro with a {variable} inside

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
            }
        }
    }
}
