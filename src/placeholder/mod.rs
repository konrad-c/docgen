pub mod error;
mod stub;
pub mod collection;
pub mod generator;

use error::PlaceholderParseError;
use stub::PlaceholderStub;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum PhoneType {
    Mobile,
    Landline
}

#[derive(Copy,Clone,Debug,Eq,Hash,PartialEq)]
pub enum Placeholder {
    FirstName,
    LastName,
    FullName,
    Phone,
    Address,
    Place,
    Float,
    Int,
    Guid,
    Set
}


impl Placeholder {
    pub fn parse<'t>(placeholder: &'t str) -> Result<Placeholder, PlaceholderParseError> {
        PlaceholderStub::parse(placeholder)
            .and_then(|parsed_stub: PlaceholderStub| Placeholder::from_stub(parsed_stub))
    }

    fn from_stub(stub: PlaceholderStub) -> Result<Placeholder, PlaceholderParseError> {
        match stub.data_type.as_str() {
            "float" => Ok(Placeholder::Float),
            "int" => Ok(Placeholder::Int),
            "phone" => Ok(Placeholder::Phone),
            "set" => Ok(Placeholder::Set),
            "first_name" => Ok(Placeholder::FirstName),
            "last_name" => Ok(Placeholder::LastName),
            "full_name" => Ok(Placeholder::FullName),
            "place" => Ok(Placeholder::Place),
            "address" => Ok(Placeholder::Address),
            "guid" => Ok(Placeholder::Guid),
            unrecognised_token => Err(PlaceholderParseError { token: unrecognised_token.to_owned(), reason: String::from("Unrecognised token.") } )
        }
    }
}