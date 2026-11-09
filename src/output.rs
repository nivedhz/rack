use crate::commands::status::StatusResult;
use crate::domain::commit::CommitEntry;
use crate::error::Result;
use chrono::{Local, TimeZone};
use colored::Colorize;

//TODO: Add rack restore docs
pub fn status(status_res: StatusResult) -> Result<()> {
    println!();
    if status_res.idx_entries.is_empty() && status_res.un_idx_entries.is_empty() {
        println!("On branch 'bonkers'");
        println!("Your branch is up to date with 'baby/bonkers'.");
        println!("\nnothing to commit, working tree clean");
        return Ok(());
    }

    if !status_res.idx_entries.is_empty() {
        println!("Changes to be commited:");
        for file in status_res.idx_entries {
            println!("      {}    {}", "modified:".green(), file);
        }
        println!();
    }
    if !status_res.un_idx_entries.is_empty() {
        println!("Changes not staged for commit:");
        println!("(use rack insert <file> to update what will be banged)");
        for file in status_res.un_idx_entries {
            println!("      {}    {}", "modified:".red(), file);
        }
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
