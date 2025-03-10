use service::EmailService;
use user::User;

fn main() {
    // let sender = User {
    //     name: String::from("onel"),
    //     email: String::from("onel@gmail.com"),
    // };

    // let recipient = User {
    //     name: String::from("sob"),
    //     email: String::from("sob@gmail.com"),
    // };

    // let email_service = Emailservice {
    //     subject: String::from("Exercise"),
    //     message: String::from("Write a Program that Displays an Email Structure Text"),
    //     email_sent_from: sender,
    //     email_sent_to: recipient,
    // };

    // email_service.send_email();
    // // println!("{:?}", new_mail);
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

