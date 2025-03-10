mod add;
mod input;
mod remove;
mod reverse;
use input::{add_input, remove_input, reverse_input};
use std::io;

fn main() {
    loop {
        println!("\nWhat do you want to do?");
        println!("1. Add , 2. Reverse , 3. Remove , 4. Exit");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let choice = input.trim().parse::<i32>().unwrap_or(0);

        match choice {
            1 => add_input(),
            2 => reverse_input(),
            3 => remove_input(),
            4 => {
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
