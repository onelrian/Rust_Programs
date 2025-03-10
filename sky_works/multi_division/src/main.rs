use division::div;
use input::input;
use multiplication::multi;

mod division;
mod multiplication;
mod input;
mod absolute;
mod sign;

fn main() {
    let num1 = input("Enter the first number: ");
    let num2 = input("Enter the second number: ");

    let places:usize;
    loop {
        let place = (input("Enter The Number of remainder places")) as i32;
    
        if place > 15{
            println!("Not Possible");
            return ();
        }
        else {
           places = place as usize;
           break; 
        }
        
    }

    let product = multi(num1 as f64, num2 as f64);
    println!("The product of {} and {} is {}", num1, num2, product);

    let quotient = div(num1 as f64, num2 as f64, places);
    println!("The quotient of {} and {} is {}", num1, num2, quotient);


}