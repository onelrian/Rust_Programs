use std::fmt::{Debug, Display};

use measurement::Quantity;
use person::Person;

fn main() {
    let onel = Person::new(String::from("Onel"), 18, Quantity::new(68.2, String::from("Kg")), Quantity::new(74.5, String::from("m")));
    println!("{:?}",onel);
    
    let new = onel.clone().get_info();
    println!("{:?}",new);
    
    let mut onel_clone = onel.clone();
    onel_clone.set_info(String::from("Richmond"), 18, Quantity::new(74.7, String::from("kg")), Quantity::new(68.8, "cm".to_string() ));
    
    let set = onel_clone.get_info();
    println!("{:?}",set);

    println!("{}",set);

}
mod person;
mod measurement;
mod display;