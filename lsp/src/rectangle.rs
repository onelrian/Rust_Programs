use crate::shape::Shape;

#[derive(Debug)]
pub struct Rectangle {
    width: f64,
    length: f64,
}

impl Rectangle {
    pub fn new(width: f64, length: f64) -> Self {
        Rectangle { 
            width,
            length,
         }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.length
    }
}
