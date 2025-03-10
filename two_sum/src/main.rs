use input::input;
use two_sum::two_sum;

fn main() {
    println!("Hello, world!");
    let array = input();

    println!("{:?}",array);

    let sum = two_sum(array, 6);

    println!("{:?}",sum);
}
mod input;
mod two_sum;