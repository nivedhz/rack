use std::{fs, path::Path};

use crate::{
    domain::{
        object::{Object, ObjectType},
        object_id::ObjectId,
        repository::Repository,
    },
    error::Result,
    storage::fsutil,
};

pub fn write_blob(repo: &Repository, path: &Path) -> Result<()> {
    let file_contents = fsutil::read_file_to_string(path)?;
    let object = Object::new(
        ObjectId::from_bytes(file_contents.as_bytes()),
        ObjectType::Blob,
        file_contents.into_bytes(),
    )?;

    match fs::create_dir(repo.object_entry_dir(&object.id)) {
        Ok(_) => (),
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
            } else {
                return Err(e.into());
            }
        }
    }
    fsutil::write_new(&repo.object_entry_file(&object.id), &object.serialize())?;

    Ok(())
}
