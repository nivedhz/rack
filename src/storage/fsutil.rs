use std::{
    fs,
    io::{Read, Write},
    path::Path,
};

use crate::{
    domain::{ignore::IgnoreEntries, repo_path::RepoPath, repository::Repository},
    error::{Error, Result},
};

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
pub fn append_atomic(path: &Path, data: &[u8]) -> Result<()> {
    if path.is_dir() {
        return Err(Error::PathIsDir {
            path: path.display().to_string(),
        });
    }

    let mut tmp_f = tempfile::NamedTempFile::new_in(path.parent().ok_or(Error::NoParent {
        path: path.display().to_string(),
    })?)?;
    if path.exists() {
        let mut og_f = fs::File::open(path)?;
        std::io::copy(&mut og_f, &mut tmp_f)?;
    }

    tmp_f.write_all(data)?;
    tmp_f.flush()?;

    tmp_f.persist(path)?;

    Ok(())
}
pub fn replace_atomic(path: &Path, data: &[u8]) -> Result<()> {
    if path.is_dir() {
        return Err(Error::PathIsDir {
            path: path.display().to_string(),
        });
    }
    if !path.exists() {
        return Err(Error::PathNotFound {
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
pub fn collect_files(root: &Path, repo: &Repository, ig: &IgnoreEntries) -> Result<Vec<RepoPath>> {
    let mut files: Vec<RepoPath> = Vec::new();

    let dirs = fs::read_dir(root)?;
    for dir in dirs {
        let rec_path = dir?.path();

        if ig.is_ignored(repo, &rec_path)? {
            continue;
        }
        if rec_path.is_dir() {
            files.extend(collect_files(&rec_path, repo, ig)?);
        } else {
            files.push(RepoPath::new(&rec_path));
        }
    }
    Ok(files)
}
