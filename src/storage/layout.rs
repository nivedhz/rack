use std::path::{Path, PathBuf};

use crate::domain::object_id::ObjectId;

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
/// Get the object file path in the current rack repository
pub fn object(root: &Path, object_id: ObjectId) -> PathBuf {
    rack_dir(root)
        .join("goods")
        .join(object_id.shard())
        .join(object_id.rest())
}
/// Get the commit file path in the current rack repository
pub fn commit(root: &Path, commit_id: ObjectId) -> PathBuf {
    rack_dir(root)
        .join("commits")
        .join(commit_id.shard())
        .join(commit_id.rest())
}
/// Get the object directory path in the current rack repository
pub fn objects_dir(root: &Path) -> PathBuf {
    rack_dir(root).join("goods")
}
/// Get the commit directory path in the current rack repository
pub fn commits_dir(root: &Path) -> PathBuf {
    rack_dir(root).join("commits")
}
