use super::util;
use super::data;
use data::first_name::FIRST_NAME;
use data::last_name::LAST_NAME;
use data::middle_name::MIDDLE_NAME;

use lazycell::LazyCell;

#[derive(Debug,Clone)]
pub struct Name {
    first: LazyCell<String>,
    middle: LazyCell<Option<String>>,
    last: LazyCell<String>
}

impl Name {
    pub fn new() -> Name {
        Name {
            first: LazyCell::new(),
            middle: LazyCell::new(),
            last: LazyCell::new()
        }
    }

    pub fn full(&mut self) -> String {
        match self.middle() {
            Some(middle) => format!("{} {} {}", self.first(), middle, self.last()),
            None => format!("{} {}", self.first(), self.last())
        }
    }

    pub fn first(&mut self) -> String {
        self.first.borrow_with(NameGenerator::first).to_owned()
    }

    pub fn middle(&mut self) -> Option<String> {
        self.middle.borrow_with(NameGenerator::middle).to_owned()
    }

    pub fn last(&mut self) -> String {
        self.last.borrow_with(NameGenerator::last).to_owned()
    }
}

struct NameGenerator;
impl NameGenerator {
    pub fn first() -> String {
        let index: usize = util::rand_index(FIRST_NAME.len());
        return FIRST_NAME[index].to_owned();
    }

    pub fn last() -> String {
        let index: usize = util::rand_index(LAST_NAME.len());
        return LAST_NAME[index].to_owned();
    }

    fn middle() -> Option<String> {
        if rand::random() {
            let index: usize = util::rand_index(MIDDLE_NAME.len());
            Some(MIDDLE_NAME[index].to_owned())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod name_tests {
    use super::*;

    #[test]
    fn test_borrow_with_already_filled() {
        let lazycell = LazyCell::new();
        let name = NameGenerator::first();
        lazycell.fill(name.clone()).unwrap();

        let value = lazycell.borrow_with(NameGenerator::first).to_owned();
        assert_eq!(name, value);
    }

    #[test]
    fn retrieving_name_twice_returns_same() {
        let name: &mut Name = &mut Name::new();
        assert_eq!(name.first(), name.first());
    }
}