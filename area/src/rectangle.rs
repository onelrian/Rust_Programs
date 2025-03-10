use crate::traits::{area::Area, form_data::FormData};
use std::io::{self, Error, ErrorKind};

#[derive(Debug)]
pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn form(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    pub fn set_width(&mut self, width: f64) {
        self.width = width;
    }

    pub fn get_width(&self) -> f64 {
        self.width
    }

    pub fn set_height(&mut self, height: f64) {
        self.height = height;
    }

    pub fn get_height(&self) -> f64 {
        self.height
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl FormData for Rectangle {
    fn collect_data(&mut self) -> Result<(), std::io::Error> {
        println!("Enter the rectangle width:");
        let mut width = String::new();
        io::stdin().read_line(&mut width)?;
        let width = match width.trim().parse() {
            Ok(value) => value,
            Err(_) => return Err(Error::new(ErrorKind::InvalidInput, "ohh!! , Sorry try again!")),
        };

        println!("Enter the rectangle height:");
        let mut height = String::new();
        io::stdin().read_line(&mut height)?;
        let height = match height.trim().parse() {
            Ok(value) => value,
            Err(_) => return Err(Error::new(ErrorKind::InvalidInput, "ohh!! , Sorry try again!")),
        };

        self.width = width;
        self.height = height;
        Ok(())
    }

    fn new() -> Self {
        Self { width: 0.0, height: 0.0 }
    }
}