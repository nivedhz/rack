use anyhow::Result;
use std::path::PathBuf;

use crate::storage::{self, bootstrap::InitStatus};

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
