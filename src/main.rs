mod generator;

use crate::generator::password_generator::Password;
use std::io::stdin;

fn main() {
    let mut prefix: String = String::new();
    let mut input_length: String = String::new();
    let length: u8;

    println!("Please enter a valid password prefix:");

    stdin()
        .read_line(&mut prefix)
        .expect("No input prefix value received!");

    loop {
        println!("\nPlease enter a valid length (u8):");

        stdin()
            .read_line(&mut input_length)
            .expect("No input length value received!");

        match input_length.trim().parse::<u8>() {
            Ok(input) => {
                length = input;
                break;
            }
            Err(_) => {
                println!("Invalid input value, please enter a valid length!");
                input_length.clear();
            }
        }
    }

    let mut password: Password = Password::new(&prefix, &length);

    println!("\nPassword: {}", password.generate());
}
