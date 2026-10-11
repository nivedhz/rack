use std::{env, path::Path};

use inquire::{Text, validator::Validation};

use crate::args::NewFlags;

pub fn initialize_rev_dir(flags: NewFlags) {
    println!();
    let command = NewCommand::new(flags);
    let name = command.get_name();
    let description = command.get_description();
    println!("name: {}", name);
    println!("description: {}", description);
}

pub struct NewCommand {
    flags: NewFlags,
}
impl NewCommand {
    fn new(flags: NewFlags) -> Self {
        Self { flags }
    }
    fn get_name(&self) -> String {
        if self.flags.default {
            format!(
                "{}",
                env::current_dir()
                    .expect("Failed to get current directory")
                    .file_name()
                    .expect("Failed to get current directory name")
                    .display()
            )
        } else {
            match self.flags.path.clone() {
                Some(name) => {
                    if name != Path::new(".") {
                        name.file_name()
                            .expect("Failed to get current directory name")
                            .display()
                            .to_string()
                    } else {
                        env::current_dir()
                            .expect("Failed to get current directory")
                            .file_name()
                            .expect("Failed to get current directory name")
                            .display()
                            .to_string()
                    }
                }
                None => {
                    let name = Text::new("Name: ")
                        .with_placeholder("my project")
                        .prompt()
                        .expect("Prompt cancelled by the user");
                    println!();
                    if name.is_empty() || name == "." {
                        env::current_dir()
                            .expect("Failed to get current directory")
                            .file_name()
                            .expect("Failed to get current directory name")
                            .display()
                            .to_string()
                    } else {
                        name
                    }
                }
            }
        }
    }
    fn get_description(&self) -> String {
        let desc_validation = |input: &str| {
            if input.chars().count() > 140 {
                Ok(Validation::Invalid(
                    "You're only allowed 140 characters".into(),
                ))
            } else {
                Ok(Validation::Valid)
            }
        };

        if self.flags.default {
            format!("Default description generated for \"{}\"", self.get_name())
        } else {
            let description = Text::new("Description")
                .with_validator(desc_validation)
                .prompt()
                .expect("Prompt cancelled by the user");
            println!();
            if description.is_empty() {
                format!("Default description generated for \"{}\"", self.get_name())
            } else {
                description
            }
        }
    }
}
