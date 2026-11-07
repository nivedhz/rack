use crate::error::Result;

pub fn run(key: String, value: String) -> Result<()> {
    dbg!(key, value);
    Ok(())
}
