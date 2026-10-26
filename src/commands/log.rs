use anyhow::Result;

pub fn run(oneline: bool) -> Result<()> {
    println!("{}", oneline);
    Ok(())
}
