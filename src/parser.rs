use regex::{Regex, Captures, Match};

lazy_static! {
    pub static ref FIELD_REGEX: Regex = Regex::new("(?P<field_type>[a-zA-Z0-9_]+)(?P<args>:[a-zA-Z0-9,]*)?").unwrap();
}

pub struct Field {
    pub field_type: String,
    pub args: Vec<String>
}

pub struct FieldParseError {
    pub token: String,
    pub reason: String
}

impl FieldParseError {
    fn invalid_field(field: &str) -> FieldParseError {
        FieldParseError { 
            token: String::from(field),
            reason: String::from("Invalid field. All fields should be of the form '${field_type}' or '${field_type:arg1,arg2}' in the case of fields with optional parameters")
        }
    }
}

pub fn parse_field<'t>(field: &'t str) -> Result<Field, FieldParseError> {
    let capture_groups: Result<Captures, FieldParseError> = match FIELD_REGEX.captures(field) {
        Some(captures) => Ok(captures),
        None => Err(FieldParseError::invalid_field(field))
    };
    capture_groups.map(|captures: Captures| {
        let field_type: String = captures.name("field_type").unwrap().as_str().to_owned();
        let arguments: Vec<&str> = captures.name("args")
            .map(|args_match: Match| args_match.as_str())
            .map(|args_str: &str| args_str.trim_start_matches(":"))
            .map(|args_trimmed: &str| args_trimmed.split(",").collect())
            .unwrap_or(Vec::new());
        let owned_args: Vec<String> = arguments
            .into_iter()
            .map(|s| s.to_owned())
            .collect();
        Field { field_type: field_type, args: owned_args }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_field_without_args() {
        match parse_field("test") {
            Ok(field) => {
                assert_eq!(field.field_type, "test");

                let empty_args: Vec<String> = Vec::new();
                assert_eq!(field.args, empty_args);
            },
            Err(e) => panic!(e.reason)
        }
    }
    
    #[test]
    fn parse_field_with_args() {
        match parse_field("test:1,2") {
            Ok(field) => {
                assert_eq!(field.field_type, "test");
                assert_eq!(field.args, vec!["1","2"]);
            },
            Err(e) => panic!(e.reason)
        }
    }
    
    #[test]
    fn forgive_field_without_args_but_with_arg_separator() {
        match parse_field("test:1,2") {
            Ok(field) => {
                assert_eq!(field.field_type, "test");
                assert_eq!(field.args, vec!["1","2"]);
            },
            Err(e) => panic!(e.reason)
        }
    }
}