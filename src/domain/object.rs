pub enum ObjectKind {
    Blob,
}
pub struct Object {
    pub kind: ObjectKind,
    pub data: Vec<u8>,
}
