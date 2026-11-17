use std::env;

use crate::{
    commands::diff::DiffedContent,
    domain::{
        commit::{CommitEntry, CommitId, CommitParent},
        repository::Repository,
    },
    error::Result,
    storage::{baby_file, commit_file, head_file, object_file},
};

// TODO: clean this shi up
pub struct ShowResult {
    pub log: Vec<CommitEntry>,
    pub diff: Vec<DiffedContent>,
}
pub fn run(commit_id: Option<String>) -> Result<ShowResult> {
    let repo = Repository::discover(&env::current_dir()?)?;
    let baby = baby_file::get_baby(&repo);

    let head_path = repo.head(&baby.path_name().path());
    let head = head_file::get_commit(&head_path)?;

    let commit_id = CommitId::new(commit_id.unwrap_or(head.hash()))?;
    let commit = commit_file::get_entry(&repo, &commit_id)?;

    let diff = match commit.parent() {
        CommitParent::Parent(id) => {
            let prev_commit = commit_file::get_entry(&repo, &id)?;
            get_diff_with_parent(&commit, &prev_commit, &repo)?
        }
        CommitParent::Orphan => get_diff_with_orphan(&commit, &repo)?,
    };

    Ok(ShowResult {
        log: vec![commit],
        diff,
    })
}

fn get_diff_with_orphan(
    curr_commit: &CommitEntry,
    repo: &Repository,
) -> Result<Vec<DiffedContent>> {
    let mut diff_entries: Vec<DiffedContent> = Vec::new();
    for (_, entry) in curr_commit.idx().entries() {
        let curr_obj = object_file::get_object_entry(repo, entry.object_id)?;

        let curr_data = String::from_utf8(curr_obj.data).unwrap_or_default();
        diff_entries.push(DiffedContent {
            prev: String::new(),
            curr: curr_data,
        });
    }

    Ok(diff_entries)
}

fn get_diff_with_parent(
    curr_commit: &CommitEntry,
    prev_commit: &CommitEntry,
    repo: &Repository,
) -> Result<Vec<DiffedContent>> {
    let mut diff_entries: Vec<DiffedContent> = Vec::new();
    for (path, entry) in curr_commit.idx().entries() {
        if let Some(e) = prev_commit.idx().contains(&path) {
            if e.is_equal(&entry) {
                continue;
            }
            let prev_obj = object_file::get_object_entry(repo, e.object_id)?;
            let curr_obj = object_file::get_object_entry(repo, entry.object_id)?;

            let prev_data = String::from_utf8(prev_obj.data).unwrap_or_default();
            let curr_data = String::from_utf8(curr_obj.data).unwrap_or_default();
            diff_entries.push(DiffedContent {
                prev: prev_data,
                curr: curr_data,
            });
        } else {
            let curr_obj = object_file::get_object_entry(repo, entry.object_id)?;
            let curr_data = String::from_utf8(curr_obj.data).unwrap_or_default();
            diff_entries.push(DiffedContent {
                prev: String::new(),
                curr: curr_data,
            });
        }
    }

    Ok(diff_entries)
}
