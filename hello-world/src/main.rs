use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    // let tupple = get_tupple("Microbiology");
    // tupple.0 , tupple.1 , tupple.2
    // (subject, favy , louisy ) = tupple; 

    let (subject, favy, louisy) = get_tupple("Microbiology");

    println!("My Students are {} Graduates", subject);
    println!(
        "They are very young. Favy is {} and Louisy is {} ",
        favy, louisy
    );  


    // let x = 45;
    // let y = -26;
    // println!("The  sum of {x} and {y} is : {}", sum(x, y))
}

fn get_tupple(subject: &str) -> (&str, i32, i32) {
    (subject, 14, 17)
}

fn _sum(x: i32, y: i32) -> i32 {
    x + y
}

fn _guessing_game() {
    let secret_number: u8 = rand::rng().random_range(1..=100);

    println!("Please input your guess.");

    let mut tries: u8 = 3;
    loop {
        if tries == 0 {
            print!("your secret number is: {}", secret_number);
            return;
        }
        tries = tries - 1;

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess: u8 = guess.trim().parse().expect("Please type a number!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");

                return;
            }
        }
    }
}
