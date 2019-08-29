pub mod error;

use error::PlaceholderParseError;
use regex::{Regex, Captures, Match};

lazy_static! {
    pub static ref PLACEHOLDER_REGEX: Regex = Regex::new("(?P<data_type>(?:[a-zA-Z0-9_]+(?:::)?)+)(?P<args>:[^:]*)?$").unwrap();
}

#[derive(Clone,Debug,Hash,Eq,PartialEq)]
pub struct Placeholder {
    pub data_type: String,
    pub args: Option<String>
}

impl Placeholder {
    pub fn parse<'t>(placeholder: &'t str) -> Result<Placeholder, PlaceholderParseError> {
        let capture_groups: Result<Captures, PlaceholderParseError> = match PLACEHOLDER_REGEX.captures(placeholder) {
            Some(captures) => Ok(captures),
            None => Err(PlaceholderParseError::invalid_placeholder(placeholder))
        };
        capture_groups.map(|captures: Captures| {
            let data_type: String = captures.name("data_type").unwrap().as_str().to_owned();
            let arguments: Option<String> = Placeholder::get_args(&captures);
            Placeholder { data_type: data_type, args: arguments }
        })
    }

    fn get_args(captures: &Captures) -> Option<String> {
        captures.name("args")
            .map(|args: Match| args.as_str())
            .map(|args: &str| args.trim_start_matches(":"))
            .map(|args: &str| args.to_owned())
    }
}

#[cfg(test)]
mod placeholder_stub_tests {
    use super::*;

    #[test]
    fn parse_placeholder_stub_without_args() {
        match Placeholder::parse("test") {
            Ok(placeholder) => {
                assert_eq!(placeholder.data_type, "test");
                assert_eq!(placeholder.args, None);
            },
            Err(e) => panic!(e.reason)
        }
    }
    
    #[test]
    fn parse_placeholder_stub_with_args() {
        match Placeholder::parse("test:1,2") {
            Ok(placeholder) => {
                assert_eq!(placeholder.data_type, "test");
                assert_eq!(placeholder.args, Some(String::from("1,2")));
            },
            Err(e) => panic!(e.reason)
        }
    }
    
    #[test]
    fn forgive_placeholder_stub_without_args_but_with_arg_separator() {
        match Placeholder::parse("test:1,2") {
            Ok(placeholder) => {
                assert_eq!(placeholder.data_type, "test");
                assert_eq!(placeholder.args, Some(String::from("1,2")));
            },
            Err(e) => panic!(e.reason)
        }
    }
    
    #[test]
    fn placeholder_stub_with_subtypes_and_args() {
        match Placeholder::parse("test::example:1,2") {
            Ok(placeholder) => {
                assert_eq!(placeholder.data_type, "test::example");
                assert_eq!(placeholder.args, Some(String::from("1,2")));
            },
            Err(e) => panic!(e.reason)
        }
    }
}