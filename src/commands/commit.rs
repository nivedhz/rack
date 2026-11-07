use crate::error::Result;

// TODO:
// Ensure that index is not empty
// Create a commit entry based on the indexed files
// Add those entries to the commit entry dir
// - Header: <prev head hash>0x20<message>0x20<author name>0x20<author email>0x20<commit timestamp>\n
// - Body: <panty-list entries joined by \n>
// Clear the panty-list
// Update the current branch head
pub fn run(message: String) -> Result<()> {
    dbg!(message);
    Ok(())
}
