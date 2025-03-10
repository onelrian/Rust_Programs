use std::f64::consts::PI;
use crate::traits::Shape;
pub struct Circle {
    radius: f64
}

impl Circle {
   pub fn new(radius:f64)-> Self{
        Self {
            radius
        }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius.powf(2.0)*PI
    }
}