use crate::shape::Shape;

#[derive(Debug)]
pub struct Square {
    side: f64,
}

impl Square {
    pub fn new(side: f64) -> Self {
        Square { 
            side
         }
    }
}


impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}
