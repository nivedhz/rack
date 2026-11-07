use serde::{Deserialize, Serialize};

use crate::{
    domain::user::User,
    error::{Error, Result},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub user: User,
}
impl Config {
    pub fn change_config(&mut self, key: String, value: String) -> Result<()> {
        let (section, key) = key.split_once(".").unwrap_or((&key, &key));
        match section {
            "user" => match key {
                "name" => {
                    self.user.set_name(value);
                    Ok(())
                }
                "email" => {
                    self.user.set_email(value);
                    Ok(())
                }
                k => Err(Error::InvalidConfig {
                    message: format!("{} is not a valid key", k),
                }),
            },
            s => Err(Error::InvalidConfig {
                message: format!("{} is not a valid section", s),
            }),
        }
    }
}
