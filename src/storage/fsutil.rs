use std::{fs, io::Write, path::Path};

use crate::error::Result;

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
