use std::{
    env,
    path::{Path, PathBuf},
};

use crate::{
    domain::{commit::CommitId, object_id::ObjectId},
    error::{self, Result},
    storage::layout,
};

pub struct Repository {
    root: PathBuf,
}

impl Repository {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }
    pub fn discover(root: &Path) -> Result<Self> {
        let curr_path = env::current_dir()?.join(root);
        for ancestor in curr_path.ancestors() {
            let rack_path = layout::rack_dir(ancestor);
            if rack_path.exists() && rack_path.is_dir() {
                return Ok(Repository::new(ancestor.to_path_buf()));
            }
            if ancestor == Path::new("/") {
                return Err(error::Error::NotInRackRepo);
            }
        }
        Err(error::Error::NotInRackRepo)
    }
    pub fn root(&self) -> PathBuf {
        self.root.clone()
    }
    pub fn rack_dir(&self) -> PathBuf {
        layout::rack_dir(&self.root)
    }
    pub fn index(&self) -> PathBuf {
        layout::index(&self.root)
    }
    pub fn head(&self) -> PathBuf {
        layout::head(&self.root)
    }
    pub fn baby(&self) -> PathBuf {
        layout::baby(&self.root)
    }
    pub fn ignore_file(&self) -> PathBuf {
        layout::ignore_file(&self.root)
    }
    pub fn objects_dir(&self) -> PathBuf {
        layout::objects_dir(&self.root)
    }
    pub fn commits_dir(&self) -> PathBuf {
        layout::commits_dir(&self.root)
    }
    pub fn config(&self) -> PathBuf {
        layout::config(&self.root)
    }
    pub fn object_entry_dir(&self, object_id: &ObjectId) -> PathBuf {
        self.objects_dir().join(object_id.shard())
    }
    pub fn object_entry_file(&self, object_id: &ObjectId) -> PathBuf {
        self.object_entry_dir(object_id).join(object_id.rest())
    }
    pub fn commit_entry_dir(&self, commit_id: &CommitId) -> PathBuf {
        self.commits_dir().join(commit_id.shard())
    }
    pub fn commit_entry_file(&self, commit_id: &CommitId) -> PathBuf {
        self.commit_entry_dir(commit_id).join(commit_id.rest())
    }
}
