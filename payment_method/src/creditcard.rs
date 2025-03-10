use crate::payment::Payment;
use crate::date::Date;

pub struct CreditCard {
    pub username: String,
    pub credit_no: u128,
    deposit_date: Date,
    expiration_date: Date,
}

impl CreditCard {
    pub fn new(username: String, credit_no: u128, deposit_date: Date, expiration_date: Date) -> Self {
        CreditCard {
            username,
            credit_no,
            deposit_date,
            expiration_date,
        }
    }
}

impl Payment for CreditCard {
    fn payment_method(&self, deposited_amount: u64) {
        println!("Dear {}\nYou deposited {}\nTo card number {}", self.username, deposited_amount, self.credit_no);
        println!("On the {}-{}-{}", self.deposit_date.year(), self.deposit_date.month(), self.deposit_date.day());
        println!("This Card would Expire on the {}-{}-{}", self.expiration_date.year(), self.expiration_date.month(), self.expiration_date.day());
        println!("Thanks for using our service");
        println!("   ");
    }
}