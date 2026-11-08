use std::str::FromStr;
use std::string::ParseError;

use itertools::Itertools;

use crate::{
    domain::{index::IndexEntries, user::User},
    error::Result,
};

pub struct CommitId {
    hash: String,
}
impl CommitId {
    fn new(hash: String) -> CommitId {
        CommitId { hash }
    }
    fn random() -> CommitId {
        let mut bytes = [0u8; 32];
        rand::fill(&mut bytes);
        let hex = hex::encode(bytes);
        CommitId { hash: hex }
    }
}
