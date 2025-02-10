use std::io;
pub fn input () -> Vec<i32> {
let mut numbers: Vec<i32> = Vec::new();

println!("Enter The Data");

loop {

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("can not read line");
    
    let input = input.trim();
    if input == "done" {
        break;
    }
    match input.parse::<i32>() {
        Ok(num) => numbers.push(num),
        Err(_) => println!("is not valid"),
    }

}
numbers


}