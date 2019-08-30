use super::Placeholder;

#[derive(Debug)]
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

    pub fn invalid_arg(placeholder: &Placeholder, help_string: &str) -> PlaceholderParseError {
        let placeholder_string: String = placeholder.to_string();
        let arg_string: String = placeholder.args.clone().unwrap_or_default();
        PlaceholderParseError { 
            placeholder: String::from(&placeholder_string),
            reason: format!("Invalid argument for placeholder '{}'. Argument(s) provided '{}' were invalid. {}", &placeholder_string, &arg_string, help_string)
        }
    }
}