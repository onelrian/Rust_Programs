
use payment::Payment;
use paypal::Paypal;
use creditcard::CreditCard;
use date::Date;

fn main() {

    let d_date = Date::new(2024,12,24);
    let d2_date = Date::new(2024,12,24);

    let e_date = Date::new(2035,10,08);
    let credit_card = CreditCard::new(String::from("Onel"), 1234, d_date, e_date);
    credit_card.payment_method(10000000);

    let paypal = Paypal::new(String::from("Jane"),54622,d2_date);

    paypal.payment_method( 200000);

  
}
mod payment;
mod paypal;
mod creditcard;
mod date;