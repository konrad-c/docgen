use super::types::{PlaceholderArgs, PlaceholderType, DistributionType};
use regex::{Regex, CaptureMatches};
use std::collections::HashMap;

lazy_static! {
    pub static ref ARGS_REGEX: Regex = Regex::new("(?P<name>[a-zA-Z0-9_]+)[ ]*=[ ]*(?P<value>[^;]+)").unwrap();
}

pub struct PlaceholderArgsParser;
impl PlaceholderArgsParser {

    pub fn parse_args(placeholder_type: &PlaceholderType, args: &String) -> Option<PlaceholderArgs> {
        let keyed_args = PlaceholderArgsParser::get_keyed_args(args);
        match placeholder_type {
            PlaceholderType::Float => PlaceholderArgsParser::parse_float(&keyed_args),
            PlaceholderType::Set => PlaceholderArgsParser::parse_set(&keyed_args),
            PlaceholderType::Int => PlaceholderArgsParser::parse_int(&keyed_args),
            PlaceholderType::Distribution(DistributionType::Normal) => PlaceholderArgsParser::parse_normal(&keyed_args),
            _ => None
        }
    }

    fn get_keyed_args(args: &String) -> HashMap<String, String> {
        let mut keyed_args = HashMap::new();
        let matches: CaptureMatches = ARGS_REGEX.captures_iter(args.as_str());
        for captures in matches {
            println!("{:?}", captures);
            let name = captures.name("name").unwrap().as_str().to_owned();
            let value = captures.name("value").unwrap().as_str().to_owned();
            keyed_args.insert(name, value);
        }
        return keyed_args;
    }

    fn parse_int(args: &HashMap<String, String>) -> Option<PlaceholderArgs> {
        let min_arg = args.get("min").and_then(|val| val.parse::<i64>().ok());
        let max_arg = args.get("max").and_then(|val| val.parse::<i64>().ok());
        let rep_arg = args.get("rep").and_then(|val| val.parse::<u64>().ok());
        match (min_arg, max_arg, rep_arg) {
            (Some(min), Some(max), Some(rep)) => Some(PlaceholderArgs::IntRepeated { min: min, max: max, repeat: rep }),
            (Some(min), Some(max), None) => Some(PlaceholderArgs::Int { min: min, max: max }),
            _ => None
        }
    }

    fn parse_float(args: &HashMap<String, String>) -> Option<PlaceholderArgs> {
        let min_val = args.get("min").and_then(|min| min.parse::<f64>().ok());
        let max_val = args.get("max").and_then(|max| max.parse::<f64>().ok());
        if min_val.is_some() && max_val.is_some() {
            return Some(PlaceholderArgs::Float {
                min: min_val.unwrap(), max: max_val.unwrap() 
            });
        }
        
        return None;
    }

    fn parse_set(args: &HashMap<String, String>) -> Option<PlaceholderArgs> {
        let parsed_options: Option<Vec<String>> = args.get("options").map(|option_str: &String| 
            option_str
                .trim_start_matches("[")
                .trim_end_matches("]")
                .split(",")
                .into_iter()
                .map(|val: &str| val.trim().to_owned())
                .collect()
        );
        parsed_options.map(|options: Vec<String>| PlaceholderArgs::Set { options: options })
    }

    fn parse_normal(args: &HashMap<String, String>) -> Option<PlaceholderArgs> {
        let mean_val: Option<f64> = args.get("mean").and_then(|mean| mean.parse::<f64>().ok());
        let stddev_val: Option<f64> = args.get("stddev").and_then(|stddev| stddev.parse::<f64>().ok())
            .and_then(|val: f64| match val < 0.0 {
                true => None,
                false => Some(val)
            });
        if mean_val.is_some() && stddev_val.is_some() {
            return Some(PlaceholderArgs::Normal { mean: mean_val.unwrap(), stddev: stddev_val.unwrap() });
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_int_args() {
        let args = "min=1;max=2".to_owned();
        let parsed_args: PlaceholderArgs = PlaceholderArgsParser::parse_args(&PlaceholderType::Int, &args).unwrap();
        match parsed_args {
            PlaceholderArgs::Int { min, max } => {
                assert_eq!(min, 1);
                assert_eq!(max, 2);
            },
            arg_type => panic!("Integer args were not parsed to PlaceholderArgs::Int. Actual: {:?}", arg_type)
        }
    }

    #[test]
    fn parse_int_rep_args() {
        let args = "min=1;max=2;rep=3".to_owned();
        let parsed_args: PlaceholderArgs = PlaceholderArgsParser::parse_args(&PlaceholderType::Int, &args).unwrap();
        match parsed_args {
            PlaceholderArgs::IntRepeated { min, max, repeat } => {
                assert_eq!(min, 1);
                assert_eq!(max, 2);
                assert_eq!(repeat, 3);
            },
            arg_type => panic!("Integer args were not parsed to PlaceholderArgs::Int. Actual: {:?}", arg_type)
        }
    }

    #[test]
    fn parse_float_args() {
        let args = "min=1;max=2".to_owned();
        let parsed_args: PlaceholderArgs = PlaceholderArgsParser::parse_args(&PlaceholderType::Float, &args).unwrap();
        match parsed_args {
            PlaceholderArgs::Float { min, max } => {
                assert_eq!(min, 1f64);
                assert_eq!(max, 2f64);
            },
            arg_type => panic!("Float args were not parsed to PlaceholderArgs::Float. Actual: {:?}", arg_type)
        }
    }

    #[test]
    fn parse_set_args() {
        let args = "options=[1,2,3,4]".to_owned();
        let parsed_args: PlaceholderArgs = PlaceholderArgsParser::parse_args(&PlaceholderType::Set, &args).unwrap();
        match parsed_args {
            PlaceholderArgs::Set { options } => {
                assert_eq!(options, vec!["1", "2", "3", "4"]);
            },
            arg_type => panic!("Set args were not parsed to PlaceholderArgs::Set. Actual: {:?}", arg_type)
        }
    }

    #[test]
    fn parse_normal_args() {
        let args = "mean=1;stddev=2".to_owned();
        let parsed_args: PlaceholderArgs = PlaceholderArgsParser::parse_args(&PlaceholderType::Distribution(DistributionType::Normal), &args).unwrap();
        match parsed_args {
            PlaceholderArgs::Normal { mean, stddev } => {
                assert_eq!(mean, 1f64);
                assert_eq!(stddev, 2f64);
            },
            arg_type => panic!("Normal args were not parsed to PlaceholderArgs::Normal. Actual: {:?}", arg_type)
        }
    }
}