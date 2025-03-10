use std::io;

mod shapes;

fn main() {
    println!("Choose a shape:");
    println!("1. Circle");
    println!("2. Rectangle");

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    let choice: u32 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter 1 or 2.");
            return;
        }
    };

    match choice {
        1 => shapes::calculate_circle_area(),
        2 => shapes::calculate_rectangle_area(),
        _ => println!("Invalid choice"),
    }
}