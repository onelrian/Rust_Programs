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
