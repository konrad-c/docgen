mod location;
mod primitive;
mod name;
mod util;
mod phone;

use uuid::Uuid;
use super::{Placeholder};

pub fn synth(placeholder: Placeholder) -> String {
    match placeholder {
        Placeholder::FirstName => name::first().to_owned(),
        Placeholder::LastName => name::last().to_owned(),
        Placeholder::FullName => name::full(),
        Placeholder::Place => location::place().to_owned(),
        Placeholder::Float => primitive::float(None).to_string(),
        Placeholder::Int  => primitive::int(0, 10).to_string(),
        Placeholder::Address => location::address(),
        Placeholder::Phone => phone::phone(),
        Placeholder::Guid => Uuid::new_v4().to_hyphenated().to_string(),
        Placeholder::Set => util::from_set(&vec![String::from("A"),String::from("B"),String::from("C")])
    }
}