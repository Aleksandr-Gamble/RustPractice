// a guessing game with verbose commentary to help you learn rust

// Very few types are in scope in the prelude.
use std::io; // Bring the std:io library into scope for input
use rand::Rng; // the Rng trait defines methods
use std::cmp::Ordering; // for looking at the ordering of numbers


fn main() {

    let secret_number = rand::thread_rng() // gives a random number generator, local to the current thread, seedec by OS
        .gen_range(1,101); // inclusive on lower bound, exclusive on upper

    loop {
        println!("Select an integer from 0-100");

        let mut guess = String::new(); /* String is a type.
            It represents a growable, utf-8 encoded bit of text on the heap
            the :: syntax is an associated function associated with the type:
            like a static method in python. it returns a new, empty string.*/
        io::stdin() // this is a type: a handle. this could have beeb std::io::stdin without the use
            .read_line(&mut guess) // call read line method. 
                                // & indicates the REFERENCE is mutable
            .expect("Failed to read line"); /*
            .read_line() returns io::Resut. Its types are enumerations.
            the OK variant and the Err variant.*/
        println!("You guessed {} :)", guess);

        let guess: u8 = match guess // SHADOW guess with another "let" to change type
            .trim() // drop the trailing \n
            .parse() { // parse a string to something else, u8 in this case
                Ok(num) => num,
                Err(_) => {
                    println!("That's not a number you idiot!");
                    continue;
                }
            };

        // match an enumerante of cmp. You need Less, Equal, and Greater to be exhaustive
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low"),
            Ordering::Equal => {
                println!("YOU WIN!");
                break},
            Ordering::Greater => println!("Too high"),
        }
    }
    println!("Thank you for playing.");
}
