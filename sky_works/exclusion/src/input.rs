use std::io;

pub fn array_input() -> ([i32; 50], usize) {
    println!("Enter the size of the array:");
    let mut size_input = String::new();
    io::stdin().read_line(&mut size_input).expect("Failed To Read Input");
    let mut size = size_input.trim().parse::<usize>().expect("Invalid Input");

    if size > 50 {
        println!("Size too large, (max is 50).");
        size = 50;
    }

    let mut data: [i32; 50] = [0; 50];

    println!("Enter the array values:");
    for i in 0..size {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed To Read Input");

        match input.trim().parse::<i32>() {
            Ok(num) => {
                data[i] = num;
            }
            Err(_) => println!("Invalid Input"),
        }
    }

    (data, size)
}

pub fn target_input() -> i32 {
    println!("Enter The Number");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed To Read Input");

    let input = match input.trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid Input");
            0
        }
    };

    input
}