use std::fmt::{Debug, Display};

use traits::form_data::FormData;

use circle::Circle;
use rectangle::Rectangle;
use traits::area::Area;

mod circle;
mod rectangle;
mod traits;

fn main() {
    // Circle input and area calculation
    let mut circle = Circle::new();
    loop {
        let result = circle.collect_data();
        if result.is_ok() {
            break;
        }
        eprintln!("{:?}", result);
    }
    println!("{:?} area: {}", circle, circle.area());

    // Rectangle input and area calculation
    let mut rect = Rectangle::new();
    loop {
        let result = rect.collect_data();
        if result.is_ok() {
            break;
        }
        eprintln!("{:?}", result);
    }
    println!("{:?} area: {}", rect, rect.area());
}