use std::fmt::Display;

fn display<T: Display>(value: T) {
    println!("{}", value);
}