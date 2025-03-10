use crate::traits::{area::Area, form_data::FormData};
use std::{f64::consts::PI, io::{self, Error, ErrorKind}};

#[derive(Debug)]

pub struct Circle {
    pub radius: f64,
    
}

impl Circle {

    pub fn from(radius:f64)->Self{
        Self {radius}
    }

    pub fn set_radius(&mut self, radius:f64) {
        self.radius = radius;
    }

    pub fn get_radius(&self)-> f64{
        self.radius
    }
}

impl Area for Circle{
    fn area(&self)->f64{
        self.radius.powf(2.0)*PI
    }
}

impl FormData for Circle {
    fn collect_data(&mut self) -> Result<(), std::io::Error> {
        println!("Enter the circle radius:");
        let mut radius = String::new();
        io::stdin().read_line(&mut radius)?;
        let radius = match radius.trim().parse() {
            Ok(value) => value,
            Err(_) => return Err(Error::new(ErrorKind::InvalidInput, "Ohh !!, sorry try again!")),
        };
        
        self.radius = radius;
        Ok(())
    }

    fn new() -> Self {
        Self { radius: 0.0 }
    }
} 