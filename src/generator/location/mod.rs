mod place;
mod street;
mod street_type;

use place::PLACE;
use street::STREET;
use street_type::STREET_TYPE;
use super::util;
use super::primitive::PrimitiveGenerator;

use lazycell::LazyCell;

#[derive(Debug,Clone)]
pub struct Location {
    street_number: LazyCell<String>,
    street: LazyCell<String>,
    place: LazyCell<String>
}

impl Location {
    pub fn new() -> Location {
        Location {
            street_number: LazyCell::new(),
            street: LazyCell::new(),
            place: LazyCell::new()
        }
    }

    pub fn address(&self) -> String {
        format!("{} {}, {}", self.street_number(), self.street(), self.place())
    }

    pub fn street_number(&self) -> String {
        self.street_number.borrow_with(LocationGenerator::street_number).to_owned()
    }

    pub fn street(&self) -> String {
        self.street.borrow_with(LocationGenerator::street).to_owned()
    }
    
    pub fn place(&self) -> String {
        self.place.borrow_with(LocationGenerator::place).to_owned()
    }
}

struct LocationGenerator;
impl LocationGenerator {
    fn place() -> String {
        let index: usize = util::rand_index(PLACE.len());
        return PLACE[index].to_owned();
    }

    fn street() -> String {
        let street_name_index: usize = util::rand_index(STREET.len());
        let street_name = STREET[street_name_index];

        let street_type_index: usize = util::rand_index(STREET_TYPE.len());
        let street_type = STREET_TYPE[street_type_index];
        format!("{} {}", street_name, street_type)
    }

    fn unit_number() -> String {
        PrimitiveGenerator::int(1, 50).to_string()
    }

    fn house_number() -> String {
        PrimitiveGenerator::int(1, 500).to_string()
    }

    fn street_number() -> String {
        match rand::random() {
            true => format!("{}/{}", LocationGenerator::unit_number(), LocationGenerator::house_number()),
            false => format!("{}", LocationGenerator::house_number())
        }
    }
}