mod stub;
pub mod error;

use stub::PlaceholderStub;
use error::PlaceholderParseError;

#[derive(Debug)]
pub enum Placeholder {
    FirstName,
    LastName,
    FullName,
    Phone,
    Address,
    Place,
    Float { rounding: Option<i8> },
    Int { min: i64, max: i64 },
    Guid,
    Set { options: Vec<String> }
}

impl Placeholder {
    pub fn parse<'t>(placeholder: &'t str) -> Result<Placeholder, PlaceholderParseError> {
        PlaceholderStub::parse(placeholder)
            .and_then(|parsed_stub: PlaceholderStub| Placeholder::from_stub(parsed_stub))
    }

    fn from_stub(stub: PlaceholderStub) -> Result<Placeholder, PlaceholderParseError> {
        match stub.data_type.as_str() {
            "float" => Placeholder::float(stub),
            "int" => Placeholder::int(stub),
            "first_name" => Ok(Placeholder::FirstName),
            "last_name" => Ok(Placeholder::LastName),
            "full_name" => Ok(Placeholder::FullName),
            "place" => Ok(Placeholder::Place),
            "address" => Ok(Placeholder::Address),
            "phone" => Ok(Placeholder::Phone),
            "guid" => Ok(Placeholder::Guid),
            "set" => Placeholder::set(stub),
            unrecognised_token => Err(PlaceholderParseError { token: unrecognised_token.to_owned(), reason: String::from("Unrecognised token.") } )
        }
    }

    fn set(stub: PlaceholderStub) -> Result<Placeholder, PlaceholderParseError> {
        match &stub.args[..] {
            options if options.len() > 0 => Ok(Placeholder::Set { options: options.to_vec() }),
            _ => Err(PlaceholderParseError { token: stub.to_string(), reason: String::from("'set' placeholder requires at least one option.") } )
        }
    }

    fn float(stub: PlaceholderStub) -> Result<Placeholder, PlaceholderParseError> {
        match &stub.args[..] {
            [] => Ok(Placeholder::Float { rounding: None }),
            [rounding] => match rounding.parse::<i8>() {
                Ok(val) => Ok(Placeholder::Float { rounding: Some(val) }),
                Err(_) => Err(PlaceholderParseError::invalid_arg(stub.to_string().as_str(), "ROUNDING", "positive or negative integer", "0, 2, -5"))
            },
            _ => Err(PlaceholderParseError { token: stub.to_string(), reason: String::from("Unrecognised arguments for type 'float'.") } )
        }
    }

    fn int(stub: PlaceholderStub) -> Result<Placeholder, PlaceholderParseError> {
        match &stub.args[..] {
            [] => Ok(Placeholder::Int { min: 0, max: 10 }),
            [min,max] => match (min.parse::<i64>(), max.parse::<i64>()) {
                (Ok(min_val), Ok(max_val)) => Ok(Placeholder::Int { min: min_val, max: max_val }),
                (Err(_), _) => Err(PlaceholderParseError::invalid_arg(stub.to_string().as_str(), "MIN", "positive or negative integer", "-9125, 2, 162")),
                (_, Err(_)) => Err(PlaceholderParseError::invalid_arg(stub.to_string().as_str(), "MAX", "positive or negative integer", "-9125, 2, 162")),
            },
            _ => Err(PlaceholderParseError { token: stub.to_string(), reason: String::from("Unrecognised arguments for type 'int'.") } )
        }
    }
}