use super::error::PlaceholderParseError;
use regex::{Regex, Captures, Match};

lazy_static! {
    pub static ref PLACEHOLDER_REGEX: Regex = Regex::new("(?P<type>[a-zA-Z0-9_]+)(?P<sub_type>(?:::[a-zA-Z0-9]+)+)?(?P<args>:[^:]*)?").unwrap();
}

pub struct PlaceholderStub {
    pub data_type: String,
    pub sub_types: Vec<String>,
    pub args: Vec<String>
}

impl PlaceholderStub {
    pub fn parse<'t>(placeholder: &'t str) -> Result<PlaceholderStub, PlaceholderParseError> {
        let capture_groups: Result<Captures, PlaceholderParseError> = match PLACEHOLDER_REGEX.captures(placeholder) {
            Some(captures) => Ok(captures),
            None => Err(PlaceholderParseError::invalid_placeholder(placeholder))
        };
        capture_groups.map(|captures: Captures| {
            let data_type: String = PlaceholderStub::get_type(&captures);
            let sub_types: Vec<String> = PlaceholderStub::get_sub_types(&captures);
            let arguments: Vec<String> = PlaceholderStub::get_args(&captures);
            PlaceholderStub { data_type: data_type, sub_types: sub_types, args: arguments }
        })
    }

    fn get_type(captures: &Captures) -> String {
        captures.name("type").unwrap().as_str().to_owned()
    }

    fn get_sub_types(captures: &Captures) -> Vec<String> {
        captures.name("sub_type")
            .map(|args_match: Match| args_match.as_str())
            .map(|args_str: &str| args_str.trim_start_matches("::"))
            .map(|args_trimmed: &str| args_trimmed.split("::").collect())
            .unwrap_or(Vec::new())
            .into_iter()
            .map(|s| s.to_owned())
            .collect()
    }

    fn get_args(captures: &Captures) -> Vec<String> {
        captures.name("args")
            .map(|args_match: Match| args_match.as_str())
            .map(|args_str: &str| args_str.trim_start_matches(":"))
            .map(|args_trimmed: &str| args_trimmed.split(",").collect())
            .unwrap_or(Vec::new())
            .into_iter()
            .map(|s| s.to_owned())
            .collect()
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
    
    #[test]
    fn placeholder_stub_with_subtypes_and_args() {
        match PlaceholderStub::parse("test::example:1,2") {
            Ok(placeholder) => {
                assert_eq!(placeholder.data_type, "test");
                assert_eq!(placeholder.sub_types, vec!["example"]);
                assert_eq!(placeholder.args, vec!["1","2"]);
            },
            Err(e) => panic!(e.reason)
        }
    }
    
    #[test]
    fn placeholder_stub_with_subtypes() {
        let empty_args: Vec<String> = Vec::new();
        match PlaceholderStub::parse("test::example::type") {
            Ok(placeholder) => {
                assert_eq!(placeholder.data_type, "test");
                assert_eq!(placeholder.sub_types, vec!["example", "type"]);
                assert_eq!(placeholder.args, empty_args);
            },
            Err(e) => panic!(e.reason)
        }
    }
    
    #[test]
    fn placeholder_stub_without_subtypes() {
        let empty_args: Vec<String> = Vec::new();
        match PlaceholderStub::parse("test") {
            Ok(placeholder) => {
                assert_eq!(placeholder.data_type, "test");
                assert_eq!(placeholder.sub_types, empty_args);
                assert_eq!(placeholder.args, empty_args);
            },
            Err(e) => panic!(e.reason)
        }
    }
}