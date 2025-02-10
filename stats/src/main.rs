use std::io;

use calculation::Calculation;
use input::input;
use mean::Mean;
use median::Median;
use mode::Mode;
fn main() {
    println!("This is A stats calculator");
    let data = input();
    println!("The values you entered are :{:?}", data);
    println!("What would you like to do ?");
    println!("median:1 , mode:2 , mean:3");
let mut choice = String::new();
io::stdin()
.read_line(&mut choice)
.expect("Invalid input");
    match choice.trim() {
        "1" => 
        { let median = Median::calc(data.clone());
        println!("The Median is: {:?}", median); }

        "2" => 
        { let mode = Mode::calc(data.clone());
        println!("The Mode is: {:?} ", mode); }

        "3" =>
        { let mean = Mean::calc(data.clone());
        println!("The Mean is: {:?}", mean); }

        _ => {println!("Invalid input");}
    }


}

mod calculation;
mod input;
mod mean;
mod median;
mod mode;
