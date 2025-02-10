use std::ops::Div;

use crate::calculation::Calculation;


pub struct Mean;
impl Calculation for Mean {
    fn calc(vector:Vec<i32>) -> f64 {
        let total:f64 = {
            let mut sum:f64 = 0_f64;
            for &i in &vector {
                sum += i as f64;
            }
            sum
        };
        let length = vector.len() as f64;
        let mean = total.div(length) ;
        
        mean
    }
}