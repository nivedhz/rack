use std::{env, fs};

use chrono::Utc;

use crate::{
    domain::{
        commit::{CommitEntry, CommitId, CommitParent},
        config::Config,
        index::IndexEntries,
        repository::Repository,
    },
    error::{Error, Result},
    storage::fsutil,
};

// TODO: add the stats for file diffs
//  1 file changed, 3 insertions(+), 5 deletions(-)
pub fn run(message: String) -> Result<()> {
    let repo = Repository::discover(&env::current_dir()?)?;

    let cfg_content = fsutil::read_file_to_string(&repo.config())?;
    let cfg = Config::parse(cfg_content)?;
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
    let idx_content = fsutil::read_file_to_string(&repo.index())?;
    let idx = IndexEntries::parse(idx_content)?;
    if idx.is_empty() {
        return Err(Error::NoFileAdded {
            message: String::from("use rack add <file> to insert a file"),
        });
    }

    let head = fsutil::read_file_to_string(&repo.head_file()).unwrap_or_default();
    let parent = CommitParent::new(head)?;
    match parent {
        CommitParent::Orphan => handle_orphan(&repo, message, cfg, idx)?,
        CommitParent::Parent(commit_id) => handle_parent(&repo, message, cfg, idx, commit_id)?,
    }

    fsutil::empty_file(&repo.index())?;
    Ok(())
}

fn handle_orphan(repo: &Repository, message: String, cfg: Config, idx: IndexEntries) -> Result<()> {
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
    fs::create_dir(repo.commit_entry_dir(&commit_id))?;
    fsutil::write_atomic(&repo.commit_entry_file(&commit_id), &commit.serialize())?;
    fsutil::write_atomic(&repo.head_file(), commit_id.hash().as_bytes())?;
    println!("[bonkers {}] {}", commit_id.short(), message);

    Ok(())
}

fn handle_parent(
    repo: &Repository,
    message: String,
    cfg: Config,
    idx: IndexEntries,
    prev_commit_id: CommitId,
) -> Result<()> {
    let head_content = fsutil::read_file_to_string(&repo.commit_entry_file(&prev_commit_id))?;
    let head_commit = CommitEntry::parse(head_content, prev_commit_id.hash())?;

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

    fs::create_dir(repo.commit_entry_dir(&curr_commit_id))?;
    fsutil::write_atomic(
        &repo.commit_entry_file(&curr_commit_id),
        &curr_commit.serialize(),
    )?;
    fsutil::write_atomic(&repo.head_file(), curr_commit_id.hash().as_bytes())?;
    println!("[bonkers {}] {}", curr_commit_id.short(), message);
    Ok(())
}
