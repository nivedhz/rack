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
pub fn write_atomic(path: &Path, data: &[u8]) -> Result<()> {
    let mut f = tempfile::NamedTempFile::new_in(path.parent().ok_or(Error::NoParent {
        path: path.display().to_string(),
    })?)?;
    f.write_all(data)?;
    f.as_file().sync_all()?;
    f.persist(path)?;
    Ok(())
}
