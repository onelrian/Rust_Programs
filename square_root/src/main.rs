use point::Point;

fn main() {
    let point1 = Point { x: 20.0, y: 10.0 };
    let point2 = Point { x: 30.0, y: 40.0 };

    let sum = point1.add_points(point2);
    println!("The sum of the two points give a new Coordinate: ");
    println!("x:{} , y:{}" , sum.x , sum.y);

}

mod point;