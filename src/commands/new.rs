use std::{
    env,
    path::{Path, PathBuf},
};

use inquire::{Text, validator::Validation};

use crate::args::NewFlags;

pub fn initialize_rev_dir(dir_path: PathBuf, flags: NewFlags) {
    println!();
    let command = NewCommand::new(flags, dir_path);
    let name = command.get_name();
    let description = command.get_description();
    let path = command.get_path();
    println!("name: {}", name);
    println!("description: {}", description);
    println!("path: {}", path.display());
}

pub struct NewCommand {
    flags: NewFlags,
    dir_path: PathBuf,
}
impl NewCommand {
    fn new(flags: NewFlags, dir_path: PathBuf) -> Self {
        Self { flags, dir_path }
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
            match self.flags.name.clone() {
                Some(name) => name,
                None => {
                    let name = Text::new("Name: ")
                        .with_placeholder("my project")
                        .prompt()
                        .expect("Prompt cancelled by the user");
                    println!();
                    if name.trim().is_empty() || name.trim() == "." {
                        format!(
                            "{}",
                            env::current_dir()
                                .expect("Failed to get current directory")
                                .file_name()
                                .expect("Failed to get current directory name")
                                .display()
                        )
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
            format!(
                "Default description generated for \"{}\"",
                env::current_dir()
                    .expect("Failed to get current directory")
                    .file_name()
                    .expect("Failed to get current directory name")
                    .display()
            )
        } else {
            let description = Text::new("Description")
                .with_validator(desc_validation)
                .prompt()
                .expect("Prompt cancelled by the user");
            println!();
            if description.is_empty() {
                format!(
                    "Default description generated for \"{}\"",
                    env::current_dir()
                        .expect("Failed to get current directory")
                        .file_name()
                        .expect("Failed to get current directory name")
                        .display()
                )
            } else {
                description
            }
        }
    }
    fn get_path(&self) -> PathBuf {
        if self.flags.default || self.dir_path == Path::new(".") {
            std::path::absolute(PathBuf::from(".")).expect("Failed to get absolute path")
        } else {
            std::path::absolute(PathBuf::from(&self.dir_path)).expect("Failed to get absolute path")
        }
    }
}
