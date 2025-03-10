use std::io;

use crate::book::Book;

pub fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

pub fn input() -> Book {
    let title = read_input("Enter book's title:");
    let author = read_input("Enter book's author:");
    let isbn = read_input("Enter book's ISBN:");
    let isbn = isbn.trim().parse::<u64>().expect("Invalid Input of ISBN");

    Book::new(title, author, isbn)
}

pub fn modify(mut book: Book) -> Book {
    let title = read_input("Enter new title:");
    let author = read_input("Enter new author:");
    let isbn = read_input("Enter new ISBN:");
    let isbn = isbn.trim().parse::<u64>().expect("Invalid Input of ISBN");

    book.new_book(title, author, isbn);

    book
}