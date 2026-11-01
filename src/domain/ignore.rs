use crate::domain::repo_path::RepoPath;

pub struct IgnoreList(Vec<String>);
impl IgnoreList {
    fn parse() -> Self {
        todo!()
    }
    fn is_ignored(&self, path: &RepoPath) -> bool {
        todo!()
    }
}
