pub struct Notification;

impl Notification {
    pub fn send_notification(user_id: &str, message: &str) {
        println!("Notifikasi untuk {}: {}", user_id, message);
    }
}
