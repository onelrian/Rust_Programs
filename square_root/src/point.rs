#[derive(Debug)]
pub struct Point {
   pub x: f64,
   pub y: f64
}

impl Point {
  pub fn add_points(&self, second: Point) -> Self {
       Point {
           x: self.x + second.x,
           y: self.y + second.y,
       }
   }
}