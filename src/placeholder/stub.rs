use super::error::PlaceholderParseError;
use regex::{Regex, Captures, Match};

lazy_static! {
    pub static ref PLACEHOLDER_REGEX: Regex = Regex::new("(?P<placeholder_type>[a-zA-Z0-9_]+)(?P<args>:.*)?").unwrap();
}

pub struct PlaceholderStub {
    pub data_type: String,
    pub args: Vec<String>
}

impl PlaceholderStub {
    pub fn parse<'t>(placeholder: &'t str) -> Result<PlaceholderStub, PlaceholderParseError> {
        let capture_groups: Result<Captures, PlaceholderParseError> = match PLACEHOLDER_REGEX.captures(placeholder) {
            Some(captures) => Ok(captures),
            None => Err(PlaceholderParseError::invalid_placeholder(placeholder))
        };
        capture_groups.map(|captures: Captures| {
            let data_type: String = captures.name("placeholder_type").unwrap().as_str().to_owned();
            let arguments: Vec<&str> = captures.name("args")
                .map(|args_match: Match| args_match.as_str())
                .map(|args_str: &str| args_str.trim_start_matches(":"))
                .map(|args_trimmed: &str| args_trimmed.split(",").collect())
                .unwrap_or(Vec::new());
            let owned_args: Vec<String> = arguments
                .into_iter()
                .map(|s| s.to_owned())
                .collect();
            PlaceholderStub { data_type: data_type, args: owned_args }
        })
    }

    pub fn to_string(&self) -> String {
        match self.args.len() {
            0 => format!("${{{}}}", self.data_type),
            _ => format!("${{{}:{:?}}}", self.data_type, self.args)
        }
    }
}


#[cfg(test)]
mod placeholder_stub_tests {
    use super::*;

    #[test]
    fn parse_placeholder_stub_without_args() {
        match PlaceholderStub::parse("test") {
            Ok(placeholder) => {
                assert_eq!(placeholder.data_type, "test");

                let empty_args: Vec<String> = Vec::new();
                assert_eq!(placeholder.args, empty_args);
            },
            Err(e) => panic!(e.reason)
        }
    }
    
    #[test]
    fn parse_placeholder_stub_with_args() {
        match PlaceholderStub::parse("test:1,2") {
            Ok(placeholder) => {
                assert_eq!(placeholder.data_type, "test");
                assert_eq!(placeholder.args, vec!["1","2"]);
            },
            Err(e) => panic!(e.reason)
        }
    }
    
    #[test]
    fn forgive_placeholder_stub_without_args_but_with_arg_separator() {
        match PlaceholderStub::parse("test:1,2") {
            Ok(placeholder) => {
                assert_eq!(placeholder.data_type, "test");
                assert_eq!(placeholder.args, vec!["1","2"]);
            },
            Err(e) => panic!(e.reason)
        }
    }
}