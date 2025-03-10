use input::input;

fn main() {
    println!("Hello, world!");
let mut test = input::input();
wiggle::wiggle_sort(&mut test);

println!("The Sorted Data is:");
println!("{:?}",test);

}

mod wiggle;
mod input;
