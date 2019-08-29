use lazycell::LazyCell;
use uuid::Uuid;

#[derive(Debug,Clone)]
pub struct Guid(LazyCell<String>);

impl Guid {
    pub fn new() -> Guid {
        Guid( LazyCell::new() )
    }

    pub fn get(&self) -> String {
        self.0.borrow_with(Guid::generate).to_owned()
    }

    fn generate() -> String {
        Uuid::new_v4().to_hyphenated().to_string()
    }
}