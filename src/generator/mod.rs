pub mod collection;

mod location;
mod primitive;
mod name;
mod util;
mod phone;

use uuid::Uuid;
use super::placeholder::{Placeholder};
// use super::placeholder::error::PlaceholderParseError;

pub fn synth(placeholder: Placeholder) -> String {
    match placeholder.data_type.as_str() {
        "name::first" => name::first().to_owned(),
        "name::last" => name::last().to_owned(),
        "name::full" => name::full(),
        "place" => location::place().to_owned(),
        "float" => primitive::float(None).to_string(),
        "int"  => primitive::int(0, 10).to_string(),
        "address" => location::address(),
        "phone" => phone::phone(),
        "guid" => Uuid::new_v4().to_hyphenated().to_string(),
        "set" => util::from_set(&vec![String::from("A"),String::from("B"),String::from("C")]),
        _ => String::new()
    }
}