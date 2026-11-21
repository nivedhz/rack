use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use crate::{
    domain::{
        commit::CommitId, ignore::IgnoreEntries, repo_path::RepoPath, repository::Repository,
    },
    error::{Error, Result},
    storage::{commit_file, object_file},
};

pub fn write_atomic(path: &Path, data: &[u8]) -> Result<()> {
    if path.is_dir() {
        return Err(Error::PathIsDir {
            path: path.display().to_string(),
        });
    }
    let parent = path
        .parent()
        .ok_or(Error::NoParent {
            path: path.display().to_string(),
        })?
        .to_path_buf();
    if !parent.exists() {
        fs::create_dir_all(&parent)?;
    }

    let mut f = tempfile::NamedTempFile::new_in(parent)?;
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
pub fn read_file_to_bytes(path: &Path) -> Result<Vec<u8>> {
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

    let mut bytes: Vec<u8> = Vec::new();
    buf_reader.read_to_end(&mut bytes)?;

    Ok(bytes)
}
pub fn collect_paths(root: &Path, repo: &Repository, ig: &IgnoreEntries) -> Result<Vec<RepoPath>> {
    let mut paths: Vec<RepoPath> = Vec::new();

    let dirs = fs::read_dir(root)?;
    for dir in dirs {
        let rec_path = dir?.path();

        if ig.is_ignored(repo, &rec_path)? {
            continue;
        }
        if rec_path.is_dir() {
            paths.push(RepoPath::new(&rec_path));
            paths.extend(collect_files(&rec_path, repo, ig)?);
        } else {
            paths.push(RepoPath::new(&rec_path));
        }
    }
    Ok(paths)
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
pub fn empty_file(path: &Path) -> Result<()> {
    let _ = std::fs::File::options()
        .write(true)
        .truncate(true)
        .open(path)?;
    Ok(())
}
pub fn get_canonical(path: &Path) -> Result<PathBuf> {
    let mut result = PathBuf::new();

    for path in path.ancestors() {
        if path.exists() {
            result = fs::canonicalize(path)?.join(result);
            break;
        } else {
            result = PathBuf::from(path.file_name().unwrap_or_default()).join(result)
        }
    }

    Ok(result.components().collect())
}
pub fn materialize_commit(id: &CommitId, repo: &Repository, ig: &IgnoreEntries) -> Result<()> {
    let curr_file_state = collect_files(&repo.root(), repo, ig)?;
    let mut branch_head_entry = commit_file::get_entry(repo, id)?.idx();

    for file in &curr_file_state {
        if let Some(e) = branch_head_entry.contains(&file.strip_prefix(&repo.root())?) {
            let f_obj_id = object_file::get_object_id(&file.path())?;
            branch_head_entry.remove(&e.path);

            if e.object_id == f_obj_id {
                continue;
            } else {
                let prev_obj = object_file::get_object_entry(repo, e.object_id)?;
                let prev_content = String::from_utf8(prev_obj.data).unwrap_or_default();

                write_atomic(&file.path(), prev_content.as_bytes())?;
            }
        } else {
            fs::remove_file(file.path())?;
        }
    }

    // the entries in `branch_head_entry` but not in `curr_file_state` will be dealt with in here
    for (repo_path, entry) in branch_head_entry.entries() {
        let object = object_file::get_object_entry(repo, entry.object_id)?;
        write_atomic(&repo_path.path(), &object.data)?;
    }

    Ok(())
}
