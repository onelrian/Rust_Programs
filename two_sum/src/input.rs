use std::io;


pub fn input() -> Vec<i32> {
    println!("Enter the series of numbers you would like to Examine");
    println!("Enter 'done' when finished");
    let mut array: Vec<i32> = Vec::new();
    loop {
    let mut vector = String::new();


    io::stdin()
    .read_line(&mut vector)
    .expect("Invalid Input");
if vector == "done"{
    break;
}
else{
    let vector:i32 = match vector.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!(" ");
            break;
        }
    };

    array.push(vector);

    }
}
    array
}