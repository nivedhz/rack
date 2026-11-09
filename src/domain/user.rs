use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub name: Option<String>,
    pub email: Option<String>,
}
impl User {
    pub fn from_str(name: &str, email: &str) -> User {
        User {
            name: Some(name.to_string()),
            email: Some(email.to_string()),
        }
    }
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }
    pub fn set_email(&mut self, email: String) {
        self.email = Some(email);
    }
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} <{}>",
            self.name.clone().unwrap_or_default(),
            self.email.clone().unwrap_or_default()
        )
    }
}
