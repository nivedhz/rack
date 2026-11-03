use crate::domain::object_id::ObjectId;

pub enum ObjectType {
    Blob,
}
impl std::fmt::Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObjectType::Blob => write!(f, "blob"),
        }
    }
}

pub struct Object {
    hash: ObjectId,
    object_type: ObjectType,
    data: Vec<u8>,
}
