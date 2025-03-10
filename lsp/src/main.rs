mod shape;
mod rectangle;
mod square;

use rectangle::Rectangle;
use shape::Shape;
use square::Square;


fn main() {
    let rectangle = Rectangle::new(5.0, 5.0);
    let square = Square::new(5.0);


    println!("{:?}",rectangle);
    println!("Rectangle area: {}", rectangle.area());

    println!("{:?}",square);
    println!("Square area: {}", square.area());
}