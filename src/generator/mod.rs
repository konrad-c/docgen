mod location;
mod primitive;
mod name;
mod util;
mod phone;

use uuid::Uuid;
use super::placeholder::{Placeholder, PhoneType};

pub fn synth(placeholder: Placeholder) -> String {
    match placeholder {
        Placeholder::FirstName => name::first().to_owned(),
        Placeholder::LastName => name::last().to_owned(),
        Placeholder::FullName => name::full(),
        Placeholder::Place => location::place().to_owned(),
        Placeholder::Float { rounding } => primitive::float(rounding).to_string(),
        Placeholder::Int { min, max }  => primitive::int(min, max).to_string(),
        Placeholder::Address => location::address(),
        Placeholder::Phone { phone_type } => match phone_type {
            Some(PhoneType::Mobile) => phone::mobile(true),
            Some(PhoneType::Landline) => phone::landline(true),
            None => phone::phone()
        },
        Placeholder::Guid => Uuid::new_v4().to_hyphenated().to_string(),
        Placeholder::Set { options } => util::from_set(&options)
    }
}