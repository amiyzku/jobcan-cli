pub struct User {
    email: String,
    password: String,
}

impl User {
    pub fn new(email: &str, password: &str) -> User {
        User {
            email: email.to_string(),
            password: password.to_string(),
        }
    }

    pub fn email(&self) -> &str {
        self.email.as_str()
    }

    pub fn password(&self) -> &str {
        self.password.as_str()
    }
}
