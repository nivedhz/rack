use std::{
    env,
    path::{Path, PathBuf},
};

use crate::constants::DIR_PATH;

pub fn find_rack_dir() -> Option<PathBuf> {
    let current_dir = env::current_dir().ok()?;
    for ancestor in current_dir.ancestors() {
        if ancestor == Path::new("/") {
            return None;
        }
        let path = ancestor.join(DIR_PATH);
        if path.exists() && path.is_dir() {
            return Some(path);
        }
    }
    None
}
