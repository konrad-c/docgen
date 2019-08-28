mod country_code;

use country_code::COUNTRY_CODE;
use super::util;
use super::primitive::{PrimitiveGenerator};
use lazycell::LazyCell;

#[derive(Debug, Clone)]
pub struct Phone {
    mobile: LazyCell<String>,
    landline: LazyCell<String>
}

impl Phone {
    pub fn new() -> Phone {
        Phone {
            mobile: LazyCell::new(),
            landline: LazyCell::new()
        }
    }

    pub fn mobile(&self) -> String {
        self.mobile.borrow_with(|| PhoneGenerator::mobile(false)).to_owned()
    }

    pub fn landline(&self) -> String {
        self.landline.borrow_with(|| PhoneGenerator::landline(false)).to_owned()
    }

    pub fn phone(&self) -> String {
        match rand::random() {
            true => self.mobile(),
            false => self.landline()
        }
    }
}

struct PhoneGenerator;
impl PhoneGenerator {
    fn country_code() -> &'static str {
        let index: usize = util::rand_index(COUNTRY_CODE.len());
        return COUNTRY_CODE[index];
    }

    fn mobile(use_country_code: bool) -> String {
        let code: String = match use_country_code {
            true => format!("({})", PhoneGenerator::country_code()),
            false => String::from("0")
        };
        format!("{}4{:02} {:03} {:03}", code, PrimitiveGenerator::int(0, 10e1 as i64), PrimitiveGenerator::int(0, 10e2 as i64), PrimitiveGenerator::int(0, 10e2 as i64))
    }

    fn landline(use_country_code: bool) -> String {
        let number: String = format!("9{:03} {:04}", PrimitiveGenerator::int(0, 10e2 as i64), PrimitiveGenerator::int(0, 10e3 as i64));
        match use_country_code {
            true => format!("({}) {}", PhoneGenerator::country_code(), number),
            false => number
        }
    }

    fn phone() -> String {
        let use_country_code: bool = rand::random();
        match rand::random() {
            true => PhoneGenerator::mobile(use_country_code),
            false => PhoneGenerator::landline(use_country_code)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn mobile_with_country_code() {
        let regex: Regex = Regex::new(r"^\(\+[0-9]{1,3}\)4[0-9]{2} [0-9]{3} [0-9]{3}$").unwrap();
        for _ in 1..20 {
            let generated_mobile = PhoneGenerator::mobile(true);
            assert!(regex.is_match(&generated_mobile), "{} did not match mobile with country code regex", generated_mobile);
        }
    }

    #[test]
    fn mobile_without_country_code() {
        let regex: Regex = Regex::new(r"^04[0-9]{2} [0-9]{3} [0-9]{3}$").unwrap();
        for _ in 1..20 {
            let generated_mobile = PhoneGenerator::mobile(false);
            assert!(regex.is_match(&generated_mobile), "{} did not match mobile without country code regex", generated_mobile);
        }
    }
    
    #[test]
    fn landline_with_country_code() {
        let regex: Regex = Regex::new(r"^\(\+[0-9]{1,3}\) [0-9]{4} [0-9]{4}$").unwrap();
        for _ in 1..20 {
            let generated_landline = PhoneGenerator::landline(true);
            assert!(regex.is_match(&generated_landline), "{} did not match landline with country code regex", generated_landline);
        }
    }
    
    #[test]
    fn landline_without_country_code() {
        let regex: Regex = Regex::new(r"^[0-9]{4} [0-9]{4}$").unwrap();
        for _ in 1..20 {
            let generated_landline = PhoneGenerator::landline(false);
            assert!(regex.is_match(&generated_landline), "{} did not match landline without country code regex", generated_landline);
        }
    }
}