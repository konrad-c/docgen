#[derive(Debug, Clone)]
pub struct PlaceholderParseError {
    pub placeholder: String,
    pub reason: String
}

impl PlaceholderParseError {
    pub fn invalid_placeholder(placeholder_string: &str) -> PlaceholderParseError {
        PlaceholderParseError {
            placeholder: String::from(placeholder_string),
            reason: String::from("Invalid placeholder. All placeholders should be of the form '${placeholder_type}' or '${placeholder_type:arg1,arg2}' in the case of placeholders with optional parameters")
        }
    }

    pub fn invalid_arg(placeholder_string: &String, arg_string: &String) -> PlaceholderParseError {
        PlaceholderParseError { 
            placeholder: *placeholder_string,
            reason: format!("Invalid argument for placeholder '{}'. Argument(s) provided '{}' were invalid.", &placeholder_string, &arg_string)
        }
    }
}