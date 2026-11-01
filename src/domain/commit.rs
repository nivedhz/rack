use crate::domain::{index::IndexEntry, object_id::ObjectId, timestamp::TimeStamp, user::User};

pub struct Commit {
    pub parent: Option<ObjectId>,
    pub message: String,
    pub author: User,
    pub timestamp: TimeStamp,
    pub entries: Vec<IndexEntry>,
}
pub enum Head {
    Unborn,
    At(ObjectId),
}
