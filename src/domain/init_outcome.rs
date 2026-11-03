use crate::domain;

pub enum InitStatus {
    Initialized,
    Reinitialized,
}
pub struct InitOutcome {
    pub repository: domain::repository::Repository,
    pub status: InitStatus,
}
