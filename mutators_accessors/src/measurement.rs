use std::{clone::Clone, fmt};
#[derive(Debug,Clone)]
pub struct Quantity{
    size : f64,
    unit : String
}

impl Quantity {
    pub fn new(size:f64,unit:String) -> Self {
        Self{
            size,
            unit
        }
    }
}

impl fmt::Display for Quantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f,"{}{}",self.size,self.unit);
        Ok(())
    }
}