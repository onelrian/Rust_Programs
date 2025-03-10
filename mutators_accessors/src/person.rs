
use std::fmt;

use crate::measurement::Quantity;
#[derive(Debug,Clone)]
pub struct Person {
    name: String,
    age : u32,
    height: Quantity,
    weight: Quantity,
}

impl Person {
    pub fn new(name:String,age:u32,height:Quantity,weight:Quantity) -> Self {
        Self {
            name,
            age,
            height,
            weight,
        }
    }

    pub fn get_info(self) -> Person {
        Person {
            name: self.name.clone(),
            age: self.age,
            height: self.height.clone(),
            weight: self.weight.clone(),
        }
    }

    pub fn set_info(&mut self, new_name:String,new_age:u32,new_height:Quantity,new_weight:Quantity){
            self.name = new_name;
            self.age = new_age;
            self.height = new_height;
            self.weight = new_weight;
        }

    }

    impl fmt::Display for Person {

        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "My name is {} I am {} Years Old I am {} Tall and I weigh {}", self.name, self.age, self.weight, self.height);
            Ok(())
        }
        
    }


