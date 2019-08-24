mod places;
mod streets;

use places::PLACES;
use streets::STREETS;
use super::util;
use super::primitive;

pub fn place() -> String {
    let index: usize = util::rand_index(PLACES.len());
    return PLACES[index].to_owned();
}

pub fn street() -> String {
    let index: usize = util::rand_index(STREETS.len());
    return STREETS[index].to_owned();
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
    format!("{} {}, {}", house_num, street(), place())
}