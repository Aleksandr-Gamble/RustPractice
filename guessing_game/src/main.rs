// a guessing game with verbose commentary to help you learn rust

// Very few types are in scope in the prelude.
use std::io; // Bring the std:io library into scope for input


fn main() {
    println!("Select an integer from 0-100");
    println!("You guess:");
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
}
