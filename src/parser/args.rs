use super::types::{PlaceholderArgs, PlaceholderType, DistributionType};

pub struct PlaceholderArgsParser;
impl PlaceholderArgsParser {

    pub fn parse_args(placeholder_type: &PlaceholderType, args: &String) -> Option<PlaceholderArgs> {
        match placeholder_type {
            // PlaceholderType::Guid => Guid::generate(),
            PlaceholderType::Float => PlaceholderArgsParser::parse_float(&args),
            PlaceholderType::Set => PlaceholderArgsParser::parse_set(&args),
            PlaceholderType::Int => PlaceholderArgsParser::parse_int(&args),
            PlaceholderType::Distribution(DistributionType::Normal) => PlaceholderArgsParser::parse_normal(&args),
            _ => None
        }
    }

    fn parse_float(args: &String) -> Option<PlaceholderArgs> {
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

    fn parse_set(args: &String) -> Option<PlaceholderArgs> {
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

    fn parse_int(args: &String) -> Option<PlaceholderArgs> {
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

    fn parse_normal(args: &String) -> Option<PlaceholderArgs> {
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