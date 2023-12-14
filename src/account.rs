pub struct Account {
    email: String,
    password: String,
}

impl Account {
    pub fn new(email: String, password: String) -> Account {
        Account { email, password }
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}
