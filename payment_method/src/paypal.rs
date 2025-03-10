use crate::payment::Payment;
use crate::date::Date;
pub struct Paypal{
    username: String,
    registration_no: u128,
    deposit_date: Date
}

impl Paypal {
    pub fn new(username:String,registration_no:u128,deposit_date:Date) -> Self {
        Paypal { 
            username,
            registration_no,
            deposit_date,
         }
    }
}

impl Payment for Paypal {
    fn payment_method(&self,deposited_amount: u64) {
        println!("Dear {} \nYou deposited {}\nTo paypal account number {}", self.username, deposited_amount,self.registration_no);
        println!("On the {}-{}-{}",self.deposit_date.year(),self.deposit_date.month(),self.deposit_date.day());
        print!("Thanks for using our service");
        print!("   ");
    }
}



