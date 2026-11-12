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
pub struct IgnoreEntries {
    entries: HashSet<PathBuf>,
}

impl IgnoreEntries {
    pub fn from_string(file_content: String) -> IgnoreEntries {
        // The entries starting with # are comments and are thus ignored
        let mut entries: HashSet<PathBuf> = file_content
            .lines()
            .filter(|&entry| !entry.is_empty() && !entry.starts_with("#"))
            .map(PathBuf::from)
            .collect();
        entries.insert(PathBuf::from(".rack/"));

        IgnoreEntries { entries }
    }
    pub fn is_ignored(&self, repository: &Repository, path: &Path) -> Result<bool> {
        let path = match fs::canonicalize(path) {
            Ok(path) => path,
            // The NotFound error is handled with a custom Error::PathNotFound error.
            // The reason for which being a custom message present in Error::PathNotFound.
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
                // Every error in strip_prefix prefix is of type std::path::StripPrefixError which
                // usually occurs when the path is in a higher hierarchy than the given path which
                // is the repository root in this case and hence can be ignored and made to be a
                // return of false
                Err(_) => return Ok(false),
            };

            if self.entries.contains(ancestor) {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

impl fmt::Display for IgnoreEntries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "IgnoreFile: {{\n\tentries: {:#?}\n}}", self.entries)
    }
}
