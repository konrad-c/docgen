pub mod error;

use error::PlaceholderParseError;
use regex::{Regex, Captures, Match};

lazy_static! {
    pub static ref PLACEHOLDER_REGEX: Regex = Regex::new("(?P<data_type>(?:[a-zA-Z0-9_]+(?:::)?)+)(?P<args>:[^:]*)?$").unwrap();
}

#[derive(Clone,Debug)]
pub enum NameType {
    First,
    Last,
    Full
}

#[derive(Clone,Debug)]
pub enum LocationType {
    Place,
    Street,
    Address
}

#[derive(Clone,Debug)]
pub enum PhoneType {
    Mobile,
    Landline,
    Any
}

#[derive(Clone,Debug)]
pub enum DistributionType {
    Normal
}

#[derive(Clone,Debug)]
pub enum PlaceholderType {
    Name(NameType),
    Location(LocationType),
    Phone(PhoneType),
    Distribution(DistributionType),
    Guid,
    Float,
    Int,
    Set
}

#[derive(Clone,Debug)]
pub struct Placeholder {
    pub original_type: String,
    pub data_type: PlaceholderType,
    pub args: Option<String>
}

impl Placeholder {
    pub fn validate<'t>(placeholder: &'t str) -> Option<PlaceholderParseError> {
        match PLACEHOLDER_REGEX.captures(placeholder) {
            Some(_) => None,
            None => Some(PlaceholderParseError::invalid_placeholder(placeholder))
        }
    }

    pub fn parse<'t>(placeholder: &'t str) -> Placeholder {
        PLACEHOLDER_REGEX.captures(placeholder)
            .map(|captures: Captures| {
                let data_type: String = captures.name("data_type").unwrap().as_str().to_owned();
                let placeholder_type: PlaceholderType = Placeholder::parse_type(&data_type);
                let arguments: Option<String> = Placeholder::get_args(&captures);
                Placeholder { original_type: data_type, data_type: placeholder_type, args: arguments }
            })
            .unwrap()
    }

    fn get_args(captures: &Captures) -> Option<String> {
        captures.name("args")
            .map(|args: Match| args.as_str())
            .map(|args: &str| args.trim_start_matches(":"))
            .map(|args: &str| args.to_owned())
    }

    pub fn to_string(&self) -> String {
        format!("${{{}:{}}}", self.original_type, self.args.clone().unwrap_or_default())
    }

    fn parse_type(data_type: &String) -> PlaceholderType {
        match data_type.as_str() {
            "name::first" => PlaceholderType::Name(NameType::First),
            "name::last" => PlaceholderType::Name(NameType::Last),
            "name::full" => PlaceholderType::Name(NameType::Full),
            "phone" => PlaceholderType::Phone(PhoneType::Any),
            "phone::mobile" => PlaceholderType::Phone(PhoneType::Mobile),
            "phone::landline" => PlaceholderType::Phone(PhoneType::Landline),
            "location::place" => PlaceholderType::Location(LocationType::Place),
            "location::street" => PlaceholderType::Location(LocationType::Street),
            "location::address" => PlaceholderType::Location(LocationType::Address),
            "dist::normal" => PlaceholderType::Distribution(DistributionType::Normal),
            "guid" => PlaceholderType::Guid,
            "float" => PlaceholderType::Float,
            "int" => PlaceholderType::Int,
            "set" => PlaceholderType::Set,
            _ => PlaceholderType::Int
        }
    }
}

// #[cfg(test)]
// mod placeholder_stub_tests {
//     use super::*;

//     #[test]
//     fn parse_placeholder_stub_without_args() {
//         let placeholder = Placeholder::parse("name::first");
//         assert_eq!(placeholder.data_type, NameType::First);
//         assert_eq!(placeholder.args, None);
//     }
    
//     #[test]
//     fn parse_placeholder_stub_with_args() {
//         let placeholder = Placeholder::parse("test:1,2");
//         assert_eq!(placeholder.data_type, "test");
//         assert_eq!(placeholder.args, Some(String::from("1,2")));
//     }
    
//     #[test]
//     fn forgive_placeholder_stub_without_args_but_with_arg_separator() {
//         let placeholder = Placeholder::parse("test:1,2");
//         assert_eq!(placeholder.data_type, "test");
//         assert_eq!(placeholder.args, Some(String::from("1,2")));
//     }
    
//     #[test]
//     fn placeholder_stub_with_subtypes_and_args() {
//         let placeholder = Placeholder::parse("test::example:1,2");
//         assert_eq!(placeholder.data_type, "test::example");
//         assert_eq!(placeholder.args, Some(String::from("1,2")));
//     }
// }