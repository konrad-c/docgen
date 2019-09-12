#[derive(Debug, Clone)]
pub struct PlaceholderParseError {
    pub placeholder: String,
    pub reason: String
}

impl PlaceholderParseError {
    pub fn invalid_placeholder(placeholder_string: &str) -> PlaceholderParseError {
        PlaceholderParseError {
            placeholder: String::from(placeholder_string),
            reason: String::from("Placeholder type not supported.")
        }
    }

    pub fn invalid_arg(placeholder_string: &String, arg_string: &String) -> PlaceholderParseError {
        PlaceholderParseError { 
            placeholder: placeholder_string.clone(),
            reason: format!("Invalid argument for placeholder '{}'. Argument(s) provided '{}' were invalid.", &placeholder_string, &arg_string)
        }
    }
}