use crate::traits::Shape;

pub struct Rectangle {
    height: f64,
    width: f64
}

impl Rectangle {
   pub fn new(height:f64,width:f64)-> Self {
        Self{
            height,
            width
        }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        &self.height*&self.width
    }
}