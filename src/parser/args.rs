use super::types::{PlaceholderArgs, PlaceholderType, DistributionType};

pub struct PlaceholderArgsParser;
impl PlaceholderArgsParser {

    pub fn parse_args(placeholder_type: &PlaceholderType, args: &String) -> Option<PlaceholderArgs> {
        match placeholder_type {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_int_args() {
        let args = "1,2".to_owned();
        let parsed_args: PlaceholderArgs = PlaceholderArgsParser::parse_int(&args).unwrap();
        match parsed_args {
            PlaceholderArgs::Int { min, max } => {
                assert_eq!(min, 1);
                assert_eq!(max, 2);
            },
            arg_type => panic!("Integer args were not parsed to PlaceholderArgs::Int. Actual: {:?}", arg_type)
        }
    }

    #[test]
    fn parse_float_args() {
        let args = "1,2".to_owned();
        let parsed_args: PlaceholderArgs = PlaceholderArgsParser::parse_float(&args).unwrap();
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
        let args = "1,2,3,4".to_owned();
        let parsed_args: PlaceholderArgs = PlaceholderArgsParser::parse_set(&args).unwrap();
        match parsed_args {
            PlaceholderArgs::Set { options } => {
                assert_eq!(options, vec!["1", "2", "3", "4"]);
            },
            arg_type => panic!("Set args were not parsed to PlaceholderArgs::Set. Actual: {:?}", arg_type)
        }
    }

    #[test]
    fn parse_normal_args() {
        let args = "1,2".to_owned();
        let parsed_args: PlaceholderArgs = PlaceholderArgsParser::parse_normal(&args).unwrap();
        match parsed_args {
            PlaceholderArgs::Normal { mean, stddev } => {
                assert_eq!(mean, 1f64);
                assert_eq!(stddev, 2f64);
            },
            arg_type => panic!("Normal args were not parsed to PlaceholderArgs::Normal. Actual: {:?}", arg_type)
        }
    }
}