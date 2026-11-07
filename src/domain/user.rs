use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: Option<String>,
    pub email: Option<String>,
}
impl User {
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }
    pub fn set_email(&mut self, email: String) {
        self.email = Some(email);
    }
}
