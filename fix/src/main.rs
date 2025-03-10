#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: &String) {

    println!("{}",data.to_uppercase());
}

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(&data);
}