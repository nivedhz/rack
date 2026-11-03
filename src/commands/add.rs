use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{
    domain::{ignore_file::IgnoreFile, repository},
    error::Result,
};

pub fn run(path: PathBuf) -> Result<()> {
    let repo = repository::Repository::discover(path.clone())?;
    let file_content = fs::read_to_string(repo.ignore_file())?;
    let ig_file = IgnoreFile::entries(file_content);
    dbg!(&ig_file);
    dbg!(ig_file.is_ignored(&repo, Path::new(".git/"))?);
    dbg!(ig_file.is_ignored(&repo, Path::new("target/"))?);
    dbg!(ig_file.is_ignored(&repo, Path::new("tmp/"))?);
    dbg!(ig_file.is_ignored(&repo, Path::new("src/main.rs"))?);
    dbg!(ig_file.is_ignored(&repo, Path::new("target/debug/rack"))?);
    dbg!(ig_file.is_ignored(&repo, Path::new("target/debug/build/output.o"))?);
    Ok(())
}
