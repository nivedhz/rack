use anyhow::Result;

pub fn run(message: String) -> Result<()> {
    println!("bang message: {}", message);
    Ok(())
}
