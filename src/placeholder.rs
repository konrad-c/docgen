use regex::{Regex, Captures, Match};

lazy_static! {
    pub static ref PLACEHOLDER_REGEX: Regex = Regex::new("(?P<placeholder_type>[a-zA-Z0-9_]+)(?P<args>:.*)?").unwrap();
}

pub struct PlaceholderStub {
    pub data_type: String,
    pub args: Vec<String>
}

impl std::fmt::Display for PlaceholderStub {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.args.len() {
            0 => write!(f, "${{{}}}", self.data_type),
            _ => write!(f, "${{{}:{:?}}}", self.data_type, self.args)
        }
    }
}

#[derive(Debug)]
pub enum Placeholder {
    FirstName,
    LastName,
    FullName,
    Place,
    Float { rounding: Option<i8> },
    Int { min: i64, max: i64 },
    Address,
    Guid
}

impl Placeholder {
    fn from_stub(stub: PlaceholderStub) -> Result<Placeholder, PlaceholderParseError> {
        match (stub.data_type.as_str(), &stub.args[..])  {
            ("float", []) => Ok(Placeholder::Float { rounding: None }),
            ("float", [rounding]) => match rounding.parse::<i8>() {
                Ok(val) => Ok(Placeholder::Float { rounding: Some(val) }),
                Err(_) => Err(PlaceholderParseError::invalid_arg(stub, "ROUNDING", "positive or negative integer", "0, 2, -5"))
            },
            ("int", []) => Ok(Placeholder::Int { min: 0, max: 10 }),
            ("int", [min,max]) => match (min.parse::<i64>(), max.parse::<i64>()) {
                (Ok(min_val), Ok(max_val)) => Ok(Placeholder::Int { min: min_val, max: max_val }),
                (Err(_), _) => Err(PlaceholderParseError::invalid_arg(stub, "MIN", "positive or negative integer", "-9125, 2, 162")),
                (_, Err(_)) => Err(PlaceholderParseError::invalid_arg(stub, "MAX", "positive or negative integer", "-9125, 2, 162")),
            },
            ("firstName", []) => Ok(Placeholder::FirstName),
            ("lastName", []) => Ok(Placeholder::LastName),
            ("fullName", []) => Ok(Placeholder::FullName),
            ("place", []) => Ok(Placeholder::Place),
            ("address", []) => Ok(Placeholder::Address),
            ("guid", []) => Ok(Placeholder::Guid),
            (unrecognised_token, _) => Err(PlaceholderParseError { token: unrecognised_token.to_owned(), reason: String::from("Unrecognised token.") } )
        }
    }
}

pub struct PlaceholderParseError {
    pub token: String,
    pub reason: String
}

impl PlaceholderParseError {
    fn invalid_placeholder(placeholder_string: &str) -> PlaceholderParseError {
        PlaceholderParseError { 
            token: String::from(placeholder_string),
            reason: String::from("Invalid placeholder. All placeholders should be of the form '${placeholder_type}' or '${placeholder_type:arg1,arg2}' in the case of placeholders with optional parameters")
        }
    }

    fn invalid_arg(stub: PlaceholderStub, arg_name: &str, arg_type: &str, example: &str) -> PlaceholderParseError {
        PlaceholderParseError { 
            token: String::from(arg_name),
            reason: format!("Invalid argument for placeholder '{}'. Argument '{}' should be of type: {}. e.g. {}", stub, arg_name, arg_type, example)
        }
    }
}

pub fn parse<'t>(placeholder: &'t str) -> Result<Placeholder, PlaceholderParseError> {
    parse_stub(placeholder)
        .and_then(|stub| Placeholder::from_stub(stub))
    
}

fn parse_stub<'t>(placeholder: &'t str) -> Result<PlaceholderStub, PlaceholderParseError> {
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

#[cfg(test)]
mod placeholder_tests {
    use super::*;

    #[test]
    fn parse_placeholder_stub_without_args() {
        match parse_stub("test") {
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
        match parse_stub("test:1,2") {
            Ok(placeholder) => {
                assert_eq!(placeholder.data_type, "test");
                assert_eq!(placeholder.args, vec!["1","2"]);
            },
            Err(e) => panic!(e.reason)
        }
    }
    
    #[test]
    fn forgive_placeholder_stub_without_args_but_with_arg_separator() {
        match parse_stub("test:1,2") {
            Ok(placeholder) => {
                assert_eq!(placeholder.data_type, "test");
                assert_eq!(placeholder.args, vec!["1","2"]);
            },
            Err(e) => panic!(e.reason)
        }
    }
}