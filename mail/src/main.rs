use service::EmailService;
use user::User;

fn main() {
   
let sender = User::new(
    String::from("onel"), 
    String::from("onel.com"),
);

let reciever = User::new(
     String::from("GIS Students"),
     String::from("allstudents.com"),
);


let email_service= EmailService{};
let subject = String::from("Rust Programming");
let message = String::from("Introduction to Rust");
email_service.send_email(&sender, &reciever, subject, message);

}

mod user;
mod service;

