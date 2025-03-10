use std::io;

pub fn input(prompt:&str) -> f64 {
    println!("{}",prompt);
    let mut number = String::new();

    io::stdin()
    .read_line( &mut number)
    .expect("Failed To read Input");

    number.trim().parse::<f64>().unwrap_or(0.0)
}