use std::{fs, path::Path};

use crate::{
    error::Result,
    storage::{fsutil, layout},
};

pub fn ensure_created(root: &Path) -> Result<()> {
    fs::create_dir_all(layout::rack_dir(root))?;
    fs::create_dir_all(layout::head(root))?;
    fs::create_dir_all(layout::objects_dir(root))?;
    fs::File::create(layout::baby(root))?;
    fs::File::create(layout::ignore_file(root))?;
    fs::File::create(layout::config(root))?;
    fs::File::create(layout::branch_index(root))?;
    Ok(())
}

pub fn seed_files(root: &Path) -> Result<()> {
    const RACK_CONFIG_CONTENT: &[u8] = b"[user]\n";
    const RACK_IGNORE_CONTENT: &[u8] = b"# Enter the relative or absolute paths of the files or directories you want to ignore.\ntmp/\ntarget\n.git/";
    const BABY_CONTENT: &[u8] = b"hump: hump/baby/bonkers";
    const BRANCH_INDEX_CONTENT: &[u8] =
        b"0000000000000000000000000000000000000000000000000000000000000000 bonkers\n";

    fsutil::write_atomic(&layout::config(root), RACK_CONFIG_CONTENT)?;
    fsutil::write_atomic(&layout::ignore_file(root), RACK_IGNORE_CONTENT)?;
    fsutil::write_atomic(&layout::baby(root), BABY_CONTENT)?;
    fsutil::write_atomic(&layout::branch_index(root), BRANCH_INDEX_CONTENT)?;
    Ok(())
}
