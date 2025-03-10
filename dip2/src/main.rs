mod notification;

use notification::{EmailNotifier, SMSNotifier, NotificationService, Notifier};

fn main() {
    let email_notifier = EmailNotifier;
    let sms_notifier = SMSNotifier;

    let email_service = NotificationService::new(Box::new(email_notifier));
    let sms_service = NotificationService::new(Box::new(sms_notifier));

    email_service.send("Hello via Email!");
    sms_service.send("Hello via SMS!");
}