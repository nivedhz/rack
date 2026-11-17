use std::path::Path;

use crate::{
    domain::commit::CommitId,
    error::{Error, Result},
    storage::fsutil,
};

pub fn get_commit(head_path: &Path) -> Result<CommitId> {
    let head_content = fsutil::read_file_to_string(head_path).unwrap_or_default();

    if head_content.trim().is_empty() {
        return Err(Error::NoCommitsFound {
            message: String::from("use rack commit to add commits!"),
        });
    }

    CommitId::new(head_content)
}
