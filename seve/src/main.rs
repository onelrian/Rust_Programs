

fn main() {
    println!("Hello, world!");

    let vector = vec![4,1,2,1,2];

    println!("{:?}",checker::checker(&vector));
}
mod checker;