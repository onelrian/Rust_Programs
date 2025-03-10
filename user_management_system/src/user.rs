// pub struct User {
//     pub name: String,
//     pub email: String,
// }

// pub struct Emailservice {
//     pub subject: String,
//     pub message: String,
//     pub email_sent_to: User,
//     pub email_sent_from: User,
// }

// impl Emailservice {
//     pub fn send_email(&self) {

//         println!("Sending email ");
//         println!("From: {} , {}", self.email_sent_from.name, self.email_sent_from.email);
//         println!("To: {} , {}", self.email_sent_to.name, self.email_sent_to.email);
//         println!("Subject: {}", self.subject);
//         println!("Message: {}", self.message);

//     }
// }

pub struct User {
    name: String,
    email: String,
}

impl User {
    pub fn new(name: String, email: String) -> Self {
        Self { name, email }
    }

    pub fn _get_name(&self) -> String {
        format!("{}", self.name)
    }

    pub fn get_email(&self) -> String {
        // username <example.com>
        format!("{} <{}>", self.name, self.email)
    }
}
