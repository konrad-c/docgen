pub mod error;

use super::types::{PlaceholderType,PhoneType,NameType,LocationType,DistributionType,PlaceholderArgs};
use error::PlaceholderParseError;
use regex::{Regex, Captures, Match};

lazy_static! {
    pub static ref PLACEHOLDER_REGEX: Regex = Regex::new("(?P<data_type>(?:[a-zA-Z0-9_]+(?:::)?)+)(?P<args>:[^:]*)?$").unwrap();
}

pub trait Args {
    fn default() -> Self;
    fn help() -> &'static str;
    fn parse(args: &String) -> Option<Self> where Self: Sized;
}

#[derive(Clone,Debug)]
pub struct Placeholder {
    pub data_type: PlaceholderType,
    pub data_args: Option<PlaceholderArgs>,
    original_type: String,
    args: Option<String>
}

impl Placeholder {
    pub fn validate<'t>(placeholder: &'t str) -> Option<PlaceholderParseError> {
        let capture_option: Option<Captures> = PLACEHOLDER_REGEX.captures(placeholder);
        if let None = capture_option {
            return Some(PlaceholderParseError::invalid_placeholder(placeholder));
        }
        let captures: Captures = capture_option.unwrap();
        // Get parsed PlaceholderType
        let data_type_string: String = Placeholder::get_data_type(&captures);
        let parsed_type: Option<PlaceholderType> = Placeholder::parse_type(&data_type_string);
        if let None = parsed_type {
            return Some(PlaceholderParseError::invalid_placeholder(placeholder));
        }

        // Get parsed PlaceholderArgs
        let data_type = parsed_type.unwrap();
        let args_capture: Option<String> = Placeholder::get_args(&captures);
        if let Some(args_string) = args_capture {
            if Placeholder::parse_args(&data_type, &args_string).is_none() {
                return Some(PlaceholderParseError::invalid_arg(&placeholder.to_owned(), &args_string));
            }
        }

        None
    }

    pub fn parse<'t>(placeholder: &'t str) -> Placeholder {
        PLACEHOLDER_REGEX.captures(placeholder)
            .map(|captures: Captures| {
                let data_type: String = Placeholder::get_data_type(&captures);
                let placeholder_type: PlaceholderType = Placeholder::parse_type(&data_type).unwrap();
                let arguments: Option<String> = Placeholder::get_args(&captures);
                let placeholder_args: Option<PlaceholderArgs> = arguments.clone()
                    .and_then(|args: String| Placeholder::parse_args(&placeholder_type, &args));
                Placeholder { original_type: data_type, args: arguments, data_type: placeholder_type, data_args: placeholder_args }
            })
            .unwrap()
    }

    fn get_data_type(placeholder_captures: &Captures) -> String {
        placeholder_captures.name("data_type").unwrap().as_str().to_owned()
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

    fn parse_type(data_type: &String) -> Option<PlaceholderType> {
        match data_type.as_str() {
            "name::first" => Some(PlaceholderType::Name(NameType::First)),
            "name::last" => Some(PlaceholderType::Name(NameType::Last)),
            "name::full" => Some(PlaceholderType::Name(NameType::Full)),
            "phone" => Some(PlaceholderType::Phone(PhoneType::Any)),
            "phone::mobile" => Some(PlaceholderType::Phone(PhoneType::Mobile)),
            "phone::landline" => Some(PlaceholderType::Phone(PhoneType::Landline)),
            "location::place" => Some(PlaceholderType::Location(LocationType::Place)),
            "location::street" => Some(PlaceholderType::Location(LocationType::Street)),
            "location::address" => Some(PlaceholderType::Location(LocationType::Address)),
            "dist::normal" => Some(PlaceholderType::Distribution(DistributionType::Normal)),
            "guid" => Some(PlaceholderType::Guid),
            "float" => Some(PlaceholderType::Float),
            "int" => Some(PlaceholderType::Int),
            "set" => Some(PlaceholderType::Set),
            _ => None
        }
    }

    fn parse_args(placeholder_type: &PlaceholderType, args: &String) -> Option<PlaceholderArgs> {
        match placeholder_type {
            // PlaceholderType::Guid => Guid::generate(),
            PlaceholderType::Float => ArgsParser::parse_float(&args),
            PlaceholderType::Set => ArgsParser::parse_set(&args),
            PlaceholderType::Int => ArgsParser::parse_int(&args),
            PlaceholderType::Distribution(DistributionType::Normal) => ArgsParser::parse_normal(&args),
            _ => None
        }
    }
}

struct ArgsParser;
impl ArgsParser {
    pub fn parse_float(args: &String) -> Option<PlaceholderArgs> {
        let arg_vec: Vec<&str> = args.split(",")
            .into_iter()
            .collect();
        if let [min, max] = arg_vec[..] {
            let min_val: Result<f64, _> = min.parse::<f64>();
            let max_val: Result<f64, _> = max.parse::<f64>();
            if min_val.is_ok() && max_val.is_ok() {
                return Some(PlaceholderArgs::Float {
                    min: min_val.unwrap(), max: max_val.unwrap() 
                });
            }
        }
        
        return None;
    }

    pub fn parse_set(args: &String) -> Option<PlaceholderArgs> {
        match args.is_empty() {
            true => None,
            false => Some(PlaceholderArgs::Set { 
                options :args.split(",")
                    .into_iter()
                    .map(|s: &str| s.to_owned())
                    .collect()
            })
        }
    }

    pub fn parse_int(args: &String) -> Option<PlaceholderArgs> {
        let arg_vec: Vec<&str> = args.split(",")
            .into_iter()
            .collect();
        if let [min, max] = arg_vec[..] {
            let min_val: Result<i64, _> = min.parse::<i64>();
            let max_val: Result<i64, _> = max.parse::<i64>();
            if min_val.is_ok() && max_val.is_ok() {
                return Some(PlaceholderArgs::Int { min: min_val.unwrap(), max: max_val.unwrap() });
            }
        }
        None
    }

    pub fn parse_normal(args: &String) -> Option<PlaceholderArgs> {
        let arg_vec: Vec<&str> = args.split(",")
            .into_iter()
            .collect();
        if let [mean, stddev] = arg_vec[..] {
            let mean_val: Option<f64> = mean.parse::<f64>().ok();
            let stddev_val: Option<f64> = stddev.parse::<f64>().ok()
                .and_then(|val: f64| match val < 0.0 {
                    true => None,
                    false => Some(val)
                });
            if mean_val.is_some() && stddev_val.is_some() {
                return Some(PlaceholderArgs::Normal { mean: mean_val.unwrap(), stddev: stddev_val.unwrap() });
            }
        }
        None
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