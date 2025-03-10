

use book::Book;
use input::{input, modify, read_input};
use library::Library;

fn main() {
    // // Test creating a new Book instance from user input
    // println!("Creating a new book from user input:");
    // let book1 = input();
    // println!("New book created: {:?}", book1);

    // // Test modifying an existing Book instance
    // println!("Modifying an existing book:");
    // let mut book2 = Book::new("Old Title".to_string(), "Old Author".to_string(), 1234567890);
    // println!("Original book: {:?}", book2);
    // book2 = modify(book2);
    // println!("Modified book: {:?}", book2);

    // // Test creating a new Book instance directly
    // println!("Creating a new book directly:");
    // let book3 = Book::new("New Title".to_string(), "New Author".to_string(), 9876543210);
    // println!("New book created: {:?}", book3);

    // println!("Creating a new library instance:");
    // let mut library = Library::new(Book::new("Beekeeper".to_string(), "Author".to_string(), 1234567890), "Available".to_string());
    // println!("New library instance created: {:?}", library);

    // // Test checking out a book
    // println!("Checking out a book:");
    // library.check_out();
    // println!("Book checked out: {:?}", library);

    // // Test checking in a book
    // println!("Checking in a book:");
    // library.check_in();
    // println!("Book checked in: {:?}", library);

    // // Test checking the status of a book
    // println!("Checking the status of a book:");
    // library = library.check();
    // println!("Book status: {:?}", library);

    let iter = read_input("Enter The Amount Of Book You want To Create:");
    let iter = iter.trim().parse::<u64>().expect("Invalid Input");

    for n in 0..iter{
         // Test creating a new Book instance from user input
    println!("Creating Book{} from user input:",n);
    let book = input();
    println!("New book created: {:?}", book);
    }
}
mod book;
mod input;
mod library;