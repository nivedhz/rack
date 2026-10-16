use anyhow::Result;

pub struct BangCommand {
    message: String,
}

impl BangCommand {
    pub fn new(message: String) -> Self {
        Self { message }
    }
    pub fn run(&self) -> Result<()> {
        println!("bang message: {}", self.message);
        Ok(())
    }
}
