use anyhow::Result;

pub struct CommitCommand {
    message: String,
}

impl CommitCommand {
    pub fn new(message: String) -> Self {
        Self { message }
    }
    pub fn run(&self) -> Result<()> {
        println!("commit message: {}", self.message);
        Ok(())
    }
}
