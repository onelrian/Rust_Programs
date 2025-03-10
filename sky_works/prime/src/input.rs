use std::io;

pub fn input() -> u128 {

    println!("Enter The Test Number");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
    .expect("Failed To Read Input");

    let input:u128 = match input.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("Invalid Input");
            0
        }
    };

    input
}