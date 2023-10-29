use core::fmt;

#[derive(Clone)]
pub struct User {
    pub email: String,
    pub password: String,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Email - {} ; Password - {}", self.email, self.password)
    }
}
