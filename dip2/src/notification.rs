pub trait Notifier {
    fn send_notification(&self, message: &str);
}

pub struct EmailNotifier;

impl Notifier for EmailNotifier {
    fn send_notification(&self, message: &str) {
        println!("Sending email: {}", message);
    }
}

pub struct SMSNotifier;

impl Notifier for SMSNotifier {
    fn send_notification(&self, message: &str) {
        println!("Sending SMS: {}", message);
    }
}

pub struct NotificationService {
    notifier: Box<dyn Notifier>,
}

impl NotificationService {
    pub fn new(notifier: Box<dyn Notifier>) -> Self {
        NotificationService { notifier }
    }

    pub fn send(&self, message: &str) {
        self.notifier.send_notification(message);
    }
}