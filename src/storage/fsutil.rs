use std::{
    fs,
    io::{Read, Write},
    path::Path,
};

use crate::error::{Error, Result};

pub fn write_new(path: &Path, content: &[u8]) -> Result<()> {
    match fs::File::options()
        .create(true)
        .truncate(false)
        .write(true)
        .open(path)
    {
        Ok(mut f) => {
            f.write_all(content)?;
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
    if path.is_dir() {
        return Err(Error::PathIsDir {
            path: path.display().to_string(),
        });
    }

    let mut f = tempfile::NamedTempFile::new_in(path.parent().ok_or(Error::NoParent {
        path: path.display().to_string(),
    })?)?;

    f.write_all(data)?;
    f.as_file().sync_all()?;
    f.persist(path)?;

    Ok(())
}
pub fn append_to_file(path: &Path, data: &[u8]) -> Result<()> {
    let mut f = fs::File::options().append(true).create(true).open(path)?;
    f.write_all(data)?;

    Ok(())
}
pub fn read_file_to_string(path: &Path) -> Result<String> {
    if !path.exists() {
        return Err(Error::PathNotFound {
            path: path.display().to_string(),
        });
    }
    if path.is_dir() {
        return Err(Error::PathIsDir {
            path: path.display().to_string(),
        });
    }
    let f = std::fs::File::open(path)?;
    let mut buf_reader = std::io::BufReader::new(f);
    let mut buf = String::new();
    buf_reader.read_to_string(&mut buf)?;

    Ok(buf)
}
