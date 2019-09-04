use uuid::Uuid;

pub struct Guid;

impl Guid {
    pub fn generate() -> String {
        Uuid::new_v4().to_hyphenated().to_string()
    }
}