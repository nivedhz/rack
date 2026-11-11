use std::{fs, path::Path};

use crate::{
    domain::{
        object::{ObjectEntry, ObjectId, ObjectType},
        repository::Repository,
    },
    error::Result,
    storage::fsutil,
};

pub fn write_blob(repo: &Repository, path: &Path) -> Result<()> {
    let file_contents = fsutil::read_file_to_string(path)?;

    let obj_id = ObjectId::from_bytes(file_contents.as_bytes());
    if repo.object_entry_dir(&obj_id).exists() {
        return Ok(());
    }

    let object = ObjectEntry::new(obj_id, ObjectType::Blob, file_contents.into_bytes())?;

    fs::create_dir(repo.object_entry_dir(&object.id))?;
    fsutil::write_atomic(&repo.object_entry_file(&object.id), &object.serialize())?;

    Ok(())
}
