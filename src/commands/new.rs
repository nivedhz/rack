use std::{
    env, fs,
    path::{Path, PathBuf},
};

use inquire::{Text, validator::Validation};

use crate::{args::NewFlags, constants::DIR_PATH};

pub fn initialize_rev_dir(dir_path: PathBuf, flags: NewFlags) {
    let command = NewCommand::new(flags, dir_path);
    command.init();
}

pub struct NewCommand {
    flags: NewFlags,
    dir_path: PathBuf,
}
impl NewCommand {
    fn new(flags: NewFlags, dir_path: PathBuf) -> Self {
        Self { flags, dir_path }
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
                .with_help_message("leave empty for default description")
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
    fn init(&self) {
        println!();
        let description = self.get_description();
        let path = self.get_path().join(DIR_PATH);
        fs::create_dir_all(&path).expect("Failed to create directory");
        fs::write(path.join("description"), description).expect("Failed to write description");

        println!("Initialized a new rack repository in {}", path.display());
    }
}
