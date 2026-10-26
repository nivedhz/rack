use chrono::{DateTime, FixedOffset};
use colored::Colorize;
use itertools::Itertools;
use std::fs;

use anyhow::{Context, Result, bail};

use crate::repository;

pub fn run(oneline: bool) -> Result<()> {
    let mut curr_head_commit = repository::read_baby_bonkers_file()?;
    if curr_head_commit.trim().is_empty() {
        println!("use rack commit <file> to add a commit");
        bail!("info: no commits made yet!");
    }
    let rack_dir_path = repository::find_rack_dir().context("could not find rack dir")?;
    loop {
        let curr_commit_entry_content = fs::read_to_string(
            rack_dir_path
                .join("commit")
                .join(&curr_head_commit[..2])
                .join(&curr_head_commit[2..]),
        )?;
        let (
            curr_commit_hash,
            curr_commit_message,
            curr_commit_author,
            curr_commit_email,
            curr_commit_time_stamp,
            _,
        ) = curr_commit_entry_content
            .split("\0")
            .collect_tuple()
            .context("fatal: could not parse commit entry")?;

        if oneline {
            println!("{} {}", curr_head_commit[..10].green(), curr_commit_message);
        } else {
            let curr_commit_time_stamp: i64 = curr_commit_time_stamp
                .parse()
                .context("fatal: could not parse commit time stamp")?;
            if let Some(curr_commit_time_stamp) =
                DateTime::from_timestamp_millis(curr_commit_time_stamp)
            {
                let fixed_time_offset =
                    FixedOffset::east_opt(19800).context("fatal: could not parse offset")?;
                let local_datetime = curr_commit_time_stamp.with_timezone(&fixed_time_offset);
                let commit_message = format!("commit {}", curr_head_commit);
                println!(
                    "{}\nAuthor: {} <{}>\nDate: {}\n\n    {}",
                    commit_message.green(),
                    curr_commit_author,
                    curr_commit_email,
                    local_datetime.format("%a %b %d %H:%M:%S %Y %z"),
                    curr_commit_message,
                );
            } else {
                bail!("fatal: could not parse commit time stamp");
            };
        }

        if curr_commit_hash.trim().is_empty() {
            break;
        }
        if !oneline {
            println!();
        }
        curr_head_commit = curr_commit_hash.to_string();
    }
    Ok(())
}
