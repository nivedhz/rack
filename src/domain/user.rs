use crate::error::{Error, Result};

pub struct User {
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(name: String, email: String) -> Result<Self> {
        if name.is_empty() || email.is_empty() {
            return Err(Error::MissingConfig);
        }
        Ok(Self { name, email })
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} <{}>", self.name, self.email)
    }
}
