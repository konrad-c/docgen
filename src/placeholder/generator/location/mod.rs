mod place;
mod street;
mod street_type;

use place::PLACE;
use street::STREET;
use street_type::STREET_TYPE;
use super::util;
use super::primitive;

pub fn place() -> &'static str {
    let index: usize = util::rand_index(PLACE.len());
    return PLACE[index];
}

pub fn street() -> &'static str {
    let index: usize = util::rand_index(STREET.len());
    return STREET[index];
}

pub fn street_type() -> &'static str {
    let index: usize = util::rand_index(STREET_TYPE.len());
    return STREET_TYPE[index];
}

fn unit_number() -> String {
    primitive::int(1, 50).to_string()
}

fn house_number() -> String {
    primitive::int(1, 500).to_string()
}

pub fn address() -> String {
    let house_num: String = match rand::random() {
        true => format!("{}/{}", unit_number(), house_number()),
        false => format!("{}", house_number())
    };
    format!("{} {} {}, {}", house_num, street(), street_type(), place())
}