use std::{env, fs, path::Path};

use chrono::Utc;

use crate::{
    domain::{
        commit::{CommitEntry, CommitId, CommitParent},
        config::Config,
        index::IndexEntries,
        repository::Repository,
    },
    error::{Error, Result},
    storage::{baby_file, commit_file, config_file, fsutil, index_file},
};

// TODO: add the stats for file diffs
//  1 file changed, 3 insertions(+), 5 deletions(-)
pub fn run(message: String) -> Result<()> {
    let repo = Repository::discover(&env::current_dir()?)?;

    let cfg = config_file::get_confit(&repo)?;
    if !cfg.user.name.is_some() || !cfg.user.email.is_some() {
        return Err(Error::ConfigNotFound {
            message: String::from("use rack config <key> <value> to set config details!"),
        });
    }

    if !repo.index().exists() {
        return Err(Error::NoFileAdded {
            message: String::from("use rack add <file> to insert a file"),
        });
    }
    let idx = index_file::get_index(&repo)?;
    if idx.is_empty() {
        return Err(Error::NoFileAdded {
            message: String::from("use rack add <file> to insert a file"),
        });
    }

    let baby = baby_file::get_baby(&repo);

    let head_file = repo.head(&baby.path_name().path());

    let head = fsutil::read_file_to_string(&head_file).unwrap_or_default();
    let parent = CommitParent::new(head)?;
    match parent {
        CommitParent::Orphan => handle_orphan(&repo, message, cfg, idx, &head_file)?,
        CommitParent::Parent(commit_id) => {
            handle_parent(&repo, message, cfg, idx, commit_id, &head_file)?
        }
    }

    fsutil::empty_file(&repo.index())?;
    Ok(())
}

fn handle_orphan(
    repo: &Repository,
    message: String,
    cfg: Config,
    idx: IndexEntries,
    head_file: &Path,
) -> Result<()> {
    let timestamp = Utc::now().timestamp();
    let commit_id = CommitId::random();

    let commit = CommitEntry::new(
        commit_id.clone(),
        CommitParent::Orphan,
        message.clone(),
        cfg.user,
        timestamp,
        idx,
    );

    fs::create_dir(repo.commits_dir())?;
    commit_file::write_commit(repo, &commit, &commit_id, head_file)?;
    println!("[bonkers {}] {}", commit_id.short(), message);

    Ok(())
}

fn handle_parent(
    repo: &Repository,
    message: String,
    cfg: Config,
    idx: IndexEntries,
    prev_commit_id: CommitId,
    head_file: &Path,
) -> Result<()> {
    let head_commit = commit_file::get_entry(repo, &prev_commit_id)?;

    let timestamp = Utc::now().timestamp();
    let curr_commit_id = CommitId::random();

    let curr_commit = CommitEntry::new(
        curr_commit_id.clone(),
        CommitParent::Parent(prev_commit_id.clone()),
        message.clone(),
        cfg.user,
        timestamp,
        head_commit.entries().extend(idx),
    );

    commit_file::write_commit(repo, &curr_commit, &curr_commit_id, head_file)?;
    println!("[bonkers {}] {}", curr_commit_id.short(), message);
    Ok(())
}
