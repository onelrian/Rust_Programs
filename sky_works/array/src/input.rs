use crate::{add, remove, reverse};
use std::io;

pub fn array_input() -> ([i32; 50], usize) {
    println!("Enter the size of the array (max 50):");
    let mut size_input = String::new();
    io::stdin()
        .read_line(&mut size_input)
        .expect("Failed to read input");

    let mut size = size_input.trim().parse::<usize>().unwrap_or(0);
    if size > 50 {
        println!("Size too large; using 50.");
        size = 50;
    }

    let mut data = [0; 50];
    println!("Enter {} elements:", size);
    for i in 0..size {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        data[i] = input.trim().parse::<i32>().unwrap_or(0);
    }

    (data, size)
}

pub fn add_input() {
    let (array, size) = array_input();

    println!("Enter the value to add:");
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read input");
    let value = value.trim().parse::<i32>().unwrap_or(0);

    println!("Enter the position to add the value:");
    let mut position = String::new();
    io::stdin()
        .read_line(&mut position)
        .expect("Failed to read input");
    let position = position.trim().parse::<usize>().unwrap_or(0);
    let position = position - 1;

    let (new_size, new_array) = add::add(array, size, value, position);

    println!("The Final Array Is (size: {}):", new_size);

    println!("{:?}", &new_array[0..new_size]);
}

pub fn reverse_input() {
    let (mut array, size) = array_input();
    let (reversed_size, reversed_array) = reverse::inverse(&mut array, size);

    println!("The Reversed Array Is (size: {}):", reversed_size);
    print!("{:?}", &reversed_array[0..reversed_size]);
}

pub fn remove_input() {
    let (array, size) = array_input();

    println!("Enter the position of the element to remove:");
    let mut pos_input = String::new();
    io::stdin()
        .read_line(&mut pos_input)
        .expect("Failed to read input");
    let position = pos_input.trim().parse::<usize>().unwrap_or(0);
    let position = position - 1;


    let (new_size, updated_array) = remove::remove(array, size, position);

    println!("The Final Array After Removal (size: {}):", new_size);

    print!("{:?}", &updated_array[0..new_size]);
}
