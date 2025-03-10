use std::f64::consts::PI;
use std::io;

pub trait Area {
    fn area(&self) -> f64;
}

pub struct Circle {
    radius: f64,
}

pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

pub fn calculate_circle_area() {
    println!("Enter the radius of the circle:");
    let mut radius = String::new();
    io::stdin()
        .read_line(&mut radius)
        .expect("Failed to read input");

    let radius: f64 = match radius.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };

    let circle = Circle { radius };
    println!("Area of the circle: {:.2}", circle.area());
}

pub fn calculate_rectangle_area() {
    println!("Enter the width of the rectangle:");
    let mut width = String::new();
    io::stdin()
        .read_line(&mut width)
        .expect("Failed to read input");

    let width: f64 = match width.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };

    println!("Enter the height of the rectangle:");
    let mut height = String::new();
    io::stdin()
        .read_line(&mut height)
        .expect("Failed to read input");

    let height: f64 = match height.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };

    let rectangle = Rectangle { width, height };
    println!("Area of the rectangle: {:.2}", rectangle.area());
}