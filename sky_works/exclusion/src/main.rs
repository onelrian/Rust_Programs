
mod input;
mod exclude;

use exclude::exclude;
use input::{array_input, target_input};

fn main() {
    println!("Hello, world!");

    let (array, size) = array_input();

    let target = target_input();

    let (result, size2) = exclude(target, array, size);

    println!("The Array You Entered is {:?}", &array[0..size]);
    println!("The Result is {:?}", &result[0..size2]);
}