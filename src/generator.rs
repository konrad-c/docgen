use super::location;
use super::primitive;
use super::name;

pub fn _generate(field_type: &str, arguments: Vec<String>) -> Result<String, FieldParseError> {
    let field_generator: Result<String, FieldParseError> = match (field_type, &arguments[..])  {
        ("float", []) => Ok(primitive::float(None).to_string()),
        ("float", [rounding]) => match rounding.parse::<i8>() {
            Ok(val) => Ok(primitive::float(Some(val)).to_string()),
            Err(_) => Err(FieldParseError { token: rounding.to_owned(), reason: String::from("Could not parse rounding for float.") })
        },
        ("firstName", []) => Ok(name::first()),
        ("lastName", []) => Ok(name::last()),
        ("fullName", []) => Ok(name::full()),
        ("place", []) => Ok(location::place()),
        (unrecognised_token, _) => Err(FieldParseError { token: unrecognised_token.to_owned(), reason: String::from("Unrecognised token.") } )
    };
    return field_generator;
}