use std::path::{Path, PathBuf};

const RACK_DIR: &str = ".rack";

/// Get the current rack directory path
pub fn rack_dir(root: &Path) -> PathBuf {
    root.join(RACK_DIR)
}
/// Get the current rack panty-list index file path
pub fn index(root: &Path) -> PathBuf {
    rack_dir(root).join("panty-list")
}
/// Get the current rack head path
pub fn head(root: &Path) -> PathBuf {
    rack_dir(root).join("hump").join("baby")
}
pub fn baby(root: &Path) -> PathBuf {
    rack_dir(root).join("BABY")
}
/// Get the current rack ignore file
pub fn ignore_file(root: &Path) -> PathBuf {
    root.join(".rackignore")
}
/// Get the object directory path in the current rack repository
pub fn objects_dir(root: &Path) -> PathBuf {
    rack_dir(root).join("goods")
}
/// Get the commit directory path in the current rack repository
pub fn commits_dir(root: &Path) -> PathBuf {
    rack_dir(root).join("commits")
}
pub fn config(root: &Path) -> PathBuf {
    rack_dir(root).join("config.toml")
}
pub fn branch_index(root: &Path) -> PathBuf {
    rack_dir(root).join("BRANCHES")
}
