use crate::{
    domain::{branch::BabyEntry, repository::Repository},
    storage::fsutil,
};

pub fn get_baby(repo: &Repository) -> BabyEntry {
    let baby_content = fsutil::read_file_to_string(&repo.baby()).unwrap_or_default();
    BabyEntry::parse(baby_content)
}
