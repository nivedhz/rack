use crate::domain::repo_path::RepoPath;

pub enum ChangeKind {
    Add,
    Modify,
    Delete,
}
pub struct Change {
    pub path: RepoPath,
    pub kind: ChangeKind,
}
pub struct StatusReport {
    pub staged: Vec<Change>,
    pub unstaged: Vec<Change>,
}
impl StatusReport {
    fn is_clean(&self) -> bool {
        if !self.staged.is_empty() || !self.unstaged.is_empty() {
            return false;
        }
        true
    }
}
