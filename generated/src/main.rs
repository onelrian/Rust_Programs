use circle::Circle;
use person::Person;
use rectangle::Rectangle;
use traits::Shape;

fn main() {
    
    
    let character = Person::new(String::from("Onel"), 18);
    character.introduce();
    let test1 = (68, 1.72);
    let answer = tuples::swap_tuple(test1);
    println!("My Height and Weight in Kg and M respectively are :");
    println!("{:?}", answer);

    let test1 = (24, 123, 10);
    let answer = tuples::tuple_sum(test1);
    println!("I've been in This Program for: {:?} Days", answer);

    let circle = Circle::new(8_f64);
    let area = circle.area();
    println!("The Area of the Circle is {:?}", area);


    let rectangle = Rectangle::new(8_f64,9_f64);
    let area = rectangle.area();
    println!("The Area of the Circle is {:?}", area);

   
}
mod person;
mod tuples;
mod traits;
mod rectangle;
mod circle;
mod longest;