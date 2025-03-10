use std::{i32, io};
pub fn input() -> Vec<i32> {
    let mut data: Vec<i32> = Vec::new();

    println!("Enter Your Data");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Invalid Input");

        let input = input.trim();
        if input == "done" {
            break;
        }

        match input.parse::<i32>() {
            Ok(num) => data.push(num),
            Err(_) => println!("Is not Valid"),
        }
    }
    data
}
