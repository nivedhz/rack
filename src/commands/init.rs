use std::path::PathBuf;

use crate::{domain::init_outcome::InitStatus, error::Result, storage};

pub fn run(dir_path: PathBuf) -> Result<()> {
    let result = storage::bootstrap::initialize(&dir_path)?;

    match result.status {
        InitStatus::Initialized => {
            println!(
                "Initialized rack repository in {}",
                result.repository.rack_dir().display()
            )
        }
        InitStatus::Reinitialized => println!(
            "Reinitialized rack repository in {}",
            result.repository.rack_dir().display()
        ),
    }
    Ok(())
}
