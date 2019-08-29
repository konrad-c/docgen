#[derive(Debug)]
pub struct PlaceholderParseError {
    pub token: String,
    pub reason: String
}

impl PlaceholderParseError {
    pub fn invalid_placeholder(placeholder_string: &str) -> PlaceholderParseError {
        PlaceholderParseError { 
            token: String::from(placeholder_string),
            reason: String::from("Invalid placeholder. All placeholders should be of the form '${placeholder_type}' or '${placeholder_type:arg1,arg2}' in the case of placeholders with optional parameters")
        }
    }

    // pub fn invalid_arg(placeholder: &str, arg_name: &str, arg_type: &str, example: &str) -> PlaceholderParseError {
    //     PlaceholderParseError { 
    //         token: String::from(arg_name),
    //         reason: format!("Invalid argument for placeholder '{}'. Argument '{}' should be of type: {}. e.g. {}", placeholder, arg_name, arg_type, example)
    //     }
    // }
}