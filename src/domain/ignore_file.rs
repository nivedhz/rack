use core::fmt;
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

use crate::{
    domain::repository::Repository,
    error::{Error, Result},
};

#[derive(Debug)]
pub struct IgnoreFile {
    entries: HashSet<PathBuf>,
}

impl IgnoreFile {
    pub fn entries(file_content: String) -> IgnoreFile {
        let entries: HashSet<PathBuf> = file_content
            .lines()
            .filter(|e| !e.is_empty() && !e.starts_with("#"))
            .map(PathBuf::from)
            .collect();

        IgnoreFile { entries }
    }
    pub fn is_ignored(&self, repository: &Repository, path: &Path) -> Result<bool> {
        let path = match fs::canonicalize(path) {
            Ok(path) => path,
            Err(err) => {
                if err.kind() == std::io::ErrorKind::NotFound {
                    return Err(Error::PathNotFound {
                        path: path.display().to_string(),
                    });
                } else {
                    return Err(Error::from(err));
                }
            }
        };
        for ancestor in path.ancestors() {
            let ancestor = fs::canonicalize(ancestor)?;
            let ancestor = match ancestor.strip_prefix(repository.root()) {
                Ok(ancestor) => ancestor,
                Err(_) => return Ok(false),
            };

            if self.entries.contains(ancestor) {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

impl fmt::Display for IgnoreFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IgnoreFile: {{\n\tentries: {:#?}\n}}", self.entries)
    }
}
