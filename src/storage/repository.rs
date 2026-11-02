use std::path::{Path, PathBuf};

use crate::{
    domain::{
        commit::{Commit, Head},
        ignore::IgnoreList,
        index::Index,
        object::Object,
        object_id::ObjectId,
        repo_path::RepoPath,
        user::User,
    },
    error::Result,
    storage::layout::rack_dir,
};

pub struct Repository {
    pub root: PathBuf,
}
impl Repository {
    fn discover(start: &Path) -> Result<Self> {
        todo!()
    }
    fn discover_from_cwd() -> Result<Self> {
        todo!()
    }
    fn root(&self) -> PathBuf {
        self.root.clone()
    }
    fn rack_dir(&self) -> PathBuf {
        rack_dir(&self.root())
    }
    fn index(&self) -> Index {
        todo!()
    }
    fn write_index(&self, index: Index) -> Result<()> {
        todo!()
    }
    fn head(&self) -> Result<ObjectId> {
        todo!()
    }
    fn set_head(&self, head: Head) -> Result<()> {
        todo!()
    }
    fn read_commit(&self, id: ObjectId) -> Result<Commit> {
        todo!()
    }
    fn write_commit(&self, commit: Commit) -> Result<()> {
        todo!()
    }
    fn store_blob(&self, blob: &[u8]) -> Result<RepoPath> {
        todo!()
    }
    fn load_object(&self, id: ObjectId) -> Result<Object> {
        todo!()
    }
    fn user(&self) -> Result<User> {
        todo!()
    }
    fn ignore_list(&self) -> Result<IgnoreList> {
        todo!()
    }
    fn relative_path(&self, path: &Path) -> Result<RepoPath> {
        todo!()
    }
    fn collect_files(&self, path: &Path, ignore_list: &IgnoreList) -> Result<Vec<RepoPath>> {
        todo!()
    }
}
