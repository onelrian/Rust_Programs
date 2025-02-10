use crate::user::User;

pub struct EmailService;
impl EmailService{
    pub fn send_email(&self,sender: &User, reciever: &User,subject:String,message:String) {
println!("Email : {}",sender.get_email());
println!("Reciever : {}",reciever.get_email());
println!("Subject : {}",subject);
println!("Content : {}",message);

    }
}