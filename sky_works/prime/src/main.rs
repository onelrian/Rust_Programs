use input::input;
use prime::prime;

mod input;
mod prime;


fn main() {
    println!("Hello, world!");

    let number = input();

    prime(number);
}
