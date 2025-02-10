pub struct Email {
    pub subject: String,
    pub message: String,
    pub cc: Vec<String>,
    pub read_at: u64,
    pub send_at: u64,
    pub to: User,
    pub from: User,
}
