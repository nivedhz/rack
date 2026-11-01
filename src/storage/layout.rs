use std::path::{Path, PathBuf};

use crate::domain::object_id::ObjectId;

const RACK_DIR: &str = ".rack";

pub fn rack_dir(root: &Path) -> PathBuf {
    root.join(RACK_DIR)
}
pub fn index(root: &Path) -> PathBuf {
    rack_dir(root).join("panty-list")
}
pub fn head(root: &Path) -> PathBuf {
    rack_dir(root).join("hump").join("baby").join("bonkers")
}
pub fn ignore_file(root: &Path) -> PathBuf {
    root.join(".rackignore")
}
pub fn object(root: &Path, object_id: ObjectId) -> PathBuf {
    rack_dir(root)
        .join("goods")
        .join(object_id.shard())
        .join(object_id.rest())
}
pub fn commit(root: &Path, commit_id: ObjectId) -> PathBuf {
    rack_dir(root)
        .join("commits")
        .join(commit_id.shard())
        .join(commit_id.rest())
}
pub fn objects_dir(root: &Path) -> PathBuf {
    rack_dir(root).join("goods")
}
pub fn commits_dir(root: &Path) -> PathBuf {
    rack_dir(root).join("commits")
}
