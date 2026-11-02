use std::{fs, io::Write, path::Path};

use crate::error::{Error, Result};

pub fn write_new(path: &Path, content: &str) -> Result<()> {
    match fs::File::options()
        .create(true)
        .truncate(false)
        .write(true)
        .open(path)
    {
        Ok(mut f) => {
            f.write_all(content.as_bytes())?;
            Ok(())
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AlreadyExists {
                Ok(())
            } else {
                Err(e.into())
            }
        }
    }
}
pub fn write_atomic(path: &Path, bytes: &[u8]) -> Result<()> {
    let dir = path.parent().ok_or(Error::ParentNotFound)?;
    fs::create_dir_all(dir)?;
    let mut temp = tempfile::NamedTempFile::new_in(dir)?;
    temp.write_all(bytes)?;
    temp.as_file().sync_all()?;
    temp.persist(path)?;
    Ok(())
}
