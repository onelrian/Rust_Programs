use crate::book::Book;
#[derive(Debug,Clone)]
pub struct Library {
    book: Book,
    status: String,

}

impl Library {

    pub fn new (book:Book,status:String) -> Self{
        Self{
            book,
            status
        }
    }

    pub fn check(&mut self) -> Self{
        if self.status == "check_in"{
            Self{
                book: self.book.clone(),
                status: String::from("Checked in"),
            }
        } else {
            Self{
                book: self.book.clone(),
                status: String::from("Check Out")
            }
        }
    }

    pub fn check_in(&mut self) {
        self.status = String::from("check_in");
    }

    pub fn check_out(&mut self) {
        self.status = String::from("check_out");
    }

}