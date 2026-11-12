use std::path::Path;

use crate::{domain::commit::CommitId, error::Result, storage::fsutil};

pub fn get_commit(head_path: &Path) -> Result<CommitId> {
    let head_content = fsutil::read_file_to_string(head_path).unwrap_or_default();

    CommitId::new(head_content)
}
