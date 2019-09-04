use super::args::Args;

pub struct Float;

impl Float {
    const DEFAULT_ROUNDING: i8 = 6;

    pub fn generate(args: FloatArgs) -> f64 {
        let range: f64 = args.max - args.min;
        let rand_in_range: f64 = range * rand::random::<f64>();
        math::round::floor(rand_in_range, Float::DEFAULT_ROUNDING)
    }
}

#[derive(Debug)]
pub struct FloatArgs {
    pub min: f64,
    pub max: f64,
}

impl Args for FloatArgs {
    fn help() -> &'static str {
        ""
    }

    fn default() -> FloatArgs {
        FloatArgs {
            min: 0.0,
            max: 1.0,
        }
    }

    fn parse<'t>(args: &String) -> Option<FloatArgs> {
        let arg_vec: Vec<&str> = args.split(",")
            .into_iter()
            .collect();
        if let [min, max] = arg_vec[..] {
            let min_val: Result<f64, _> = min.parse::<f64>();
            let max_val: Result<f64, _> = max.parse::<f64>();
            if min_val.is_ok() && max_val.is_ok() {
                return Some(FloatArgs {
                    min: min_val.unwrap(), max: max_val.unwrap() 
                });
            }
        }
        
        return None;
    }
}

#[cfg(test)]
mod int_args_tests {
    use super::*;

    #[test]
    fn parse_valid_args() {
        let parsed_args = FloatArgs::parse(&"1,2".to_owned()).unwrap();
        assert_eq!(parsed_args.min, 1f64);
        assert_eq!(parsed_args.max, 2f64);
    }

    #[test]
    fn parse_invalid_args_returns_default() {
        let invalid_args = vec!["", "string", "1,2,3"];
        for arg in invalid_args {
            let parsed_args = FloatArgs::parse(&arg.to_owned());
            assert!(parsed_args.is_none());
        }
    }
}