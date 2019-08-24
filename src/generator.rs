use super::location;
use super::primitive;
use super::name;
use super::placeholder::{Placeholder, PlaceholderParseError};

pub fn generate_data(placeholder: Placeholder) -> Result<String, PlaceholderParseError> {
    match placeholder  {
        Placeholder::FirstName => Ok(name::first()),
        Placeholder::LastName => Ok(name::last()),
        Placeholder::FullName => Ok(name::full()),
        Placeholder::Place => Ok(location::place()),
        Placeholder::Float { rounding } => Ok(primitive::float(rounding).to_string()),
        Placeholder::Int { min, max }  => Ok(primitive::int(min, max).to_string()),
        _ => panic!("Data generation not supported for placeholder type: {:?}", placeholder)
    }
}