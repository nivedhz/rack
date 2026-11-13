use std::{fs, path::PathBuf};

use crate::{
    error::Result,
    storage::{bootstrap, layout},
};

pub fn run(dir_path: PathBuf) -> Result<()> {
    if !dir_path.exists() {
        fs::create_dir_all(&dir_path)?;
    }

    let root = fs::canonicalize(dir_path)?;
    if root.is_dir() && layout::rack_dir(&root).exists() {
        bootstrap::ensure_created(&root)?;
        bootstrap::seed_files(&root)?;

        println!(
            "Reinitialized rack repository in {}",
            layout::rack_dir(&root).display()
        );
        return Ok(());
    }

    let tmp = tempfile::TempDir::new_in(&root)?;
    let tmp_path = tmp.path();
    bootstrap::ensure_created(tmp_path)?;

    if let Err(e) = fs::rename(layout::rack_dir(tmp_path), layout::rack_dir(&root)) {
        fs::remove_dir_all(tmp_path)?;
        return Err(e.into());
    }
    bootstrap::seed_files(&root)?;
    println!(
        "Initialized rack repository in {}",
        layout::rack_dir(&root).display()
    );

    Ok(())
}
