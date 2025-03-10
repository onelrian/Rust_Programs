#[derive(Debug,Clone)]
pub struct Book {

    title: String,
    author: String,
    isbn: u64
}

impl Book {
    pub fn new(title:String,author:String,isbn:u64) -> Self{
        Self{
            title,
            author,
            isbn
        }
    }

    pub fn new_book(&mut self,title:String,author:String,isbn:u64){
        self.title = title;
        self.author = author;
        self.isbn = isbn;
    }
}

