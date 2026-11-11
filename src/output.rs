use crate::commands::{diff::DiffedContent, status::StatusResult};
use crate::domain::branch::{BabyEntry, BranchEntries};
use crate::domain::commit::CommitEntry;
use crate::error::Result;
use chrono::{Local, TimeZone};
use colored::Colorize;
use diffy::PatchFormatter;

// TODO: Add awareness for standalone local commits which are not pushed
pub fn status(status_res: StatusResult) -> Result<()> {
    if status_res.idx_entries.is_empty()
        && status_res.un_idx_entries.is_empty()
        && status_res.untracked_entries.is_empty()
    {
        println!();
        println!("On branch 'bonkers'");
        println!("Your branch is up to date with 'baby/bonkers'.");
        println!("\nnothing to commit, working tree clean");
        return Ok(());
    }

    if !status_res.idx_entries.is_empty() {
        println!();
        println!("Changes to be commited:");
        println!("  (use 'rack restore --staged <file>...' to unstage)");
        for file in &status_res.idx_entries {
            println!(
                "      {}    {}",
                "modified:".green(),
                file.to_string().green()
            );
        }
        println!();
    }
    if !status_res.un_idx_entries.is_empty() {
        println!();
        println!("Changes not staged for commit:");
        println!("  (use 'rack insert <file>...' to update what will be commited)");
        println!("  (use 'rack restore <file>...' to discard changes in the working directory)");
        for file in status_res.un_idx_entries {
            println!("      {}    {}", "modified:".red(), file.to_string().red());
        }
    }
    if !status_res.untracked_entries.is_empty() {
        println!();
        println!("Untracked files:");
        println!("  (use 'rack add <file>...' to include what will be commited)");
        for file in status_res.untracked_entries {
            println!("    {}", file.to_string().red());
        }
    }
    if status_res.idx_entries.is_empty() {
        println!("\nno changes added for commit (use 'rack add')")
    }
    Ok(())
}

pub fn log(oneline: bool, entries: Vec<CommitEntry>) -> Result<()> {
    for entry in entries {
        if !oneline {
            let local_offset = *Local::now().offset();
            let formatted_timestamp =
                match local_offset.timestamp_opt(entry.timestamp(), 0).single() {
                    Some(dt) => dt.format("%a %b%e %H:%M:%S %Y %z").to_string(),
                    None => String::new(),
                };

            println!("{} {}", "commit".green(), entry.id().hash().green());
            println!("Author: {}", entry.author());
            println!("Date:   {}", formatted_timestamp);
            println!("\n    {}\n", entry.message());
        } else {
            println!("{} {}", entry.id().short().green(), entry.message())
        }
    }
    Ok(())
}

pub fn diff(entries: Vec<DiffedContent>) -> Result<()> {
    let f = PatchFormatter::new().with_color();
    for entry in entries {
        let patch = diffy::create_patch(&entry.prev, &entry.curr);
        println!("{}", f.fmt_patch(&patch))
    }
    Ok(())
}

pub fn branch(baby: BabyEntry, branch_idx: BranchEntries) {
    for (path_name, _) in branch_idx.entries() {
        if baby.name() == path_name.name() {
            print!("* {}    ", path_name.name().to_string().green())
        } else {
            print!("{}    ", path_name.name());
        }
    }
    println!();
}
