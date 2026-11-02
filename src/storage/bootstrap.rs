use std::{fs, path::Path};

use crate::{
    error::Result,
    storage::{fsutil, layout},
};

pub fn initialize(root: &Path) -> Result<()> {
    if root.is_dir() {
        ensure_created(root)?;
        seed_files(root)?;
        return Ok(());
    }

    let temp = tempfile::TempDir::new_in(root)?;
    ensure_created(temp.path())?;
    let tmp_path = temp.keep();
    if let Err(e) = fs::rename(&tmp_path, layout::rack_dir(root)) {
        fs::remove_dir_all(tmp_path)?;
        println!("Error: {}", e);
        return Err(e.into());
    }
    seed_files(root)?;
    Ok(())
}

fn ensure_created(root: &Path) -> Result<()> {
    fs::create_dir_all(layout::rack_dir(root))?;
    fs::File::create(layout::baby(root))?;
    fs::create_dir_all(layout::head(root))?;
    fs::create_dir_all(layout::objects_dir(root))?;
    fs::File::create(layout::ignore_file(root))?;
    fs::File::create(layout::config(root))?;
    Ok(())
}

fn seed_files(root: &Path) -> Result<()> {
    const RACK_CONFIG_CONTENT: &str = "[user]\n";
    const RACK_IGNORE_CONTENT: &str = "# Enter the relative or absolute paths of the files or directories you want to ignore.\ntmp/\ntarget\n.git/";
    const BABY_CONTENT: &str = "hump: hump/baby/bonkers";

    fsutil::write_new(&layout::config(root), RACK_CONFIG_CONTENT)?;
    fsutil::write_new(&layout::ignore_file(root), RACK_IGNORE_CONTENT)?;
    fsutil::write_new(&layout::baby(root), BABY_CONTENT)?;
    Ok(())
}
