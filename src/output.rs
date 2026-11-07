use crate::commands::status::StatusResult;
use crate::error::Result;
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
