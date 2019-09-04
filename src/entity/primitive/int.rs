use super::args::Args;

pub struct Int;

impl Int {
    pub fn generate(args: IntArgs) -> i64 {
        let range: i64 = args.max - args.min;
        let rand_in_range: f64 = (range as f64) * rand::random::<f64>();
        return args.min + rand_in_range as i64;
    }
}

#[derive(Debug)]
pub struct IntArgs {
    pub min: i64,
    pub max: i64
}

impl Args for IntArgs {
    fn help() -> &'static str {
        ""
    }

    fn default() -> IntArgs {
        IntArgs { min: 0, max: 10 }
    }

    fn parse(args: &String) -> Option<IntArgs> {
        let arg_vec: Vec<&str> = args.split(",")
            .into_iter()
            .collect();
        if let [min, max] = arg_vec[..] {
            let min_val: Result<i64, _> = min.parse::<i64>();
            let max_val: Result<i64, _> = max.parse::<i64>();
            if min_val.is_ok() && max_val.is_ok() {
                return Some(IntArgs { min: min_val.unwrap(), max: max_val.unwrap() });
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn int_args_valid() {
        let parsed_args = IntArgs::parse(&"1,2".to_owned()).unwrap();
        assert_eq!(parsed_args.min, 1);
        assert_eq!(parsed_args.max, 2);
    }

    #[test]
    fn parse_invalid_args_returns_none() {
        let invalid_args = vec!["", "1", "2", "string", "1,2,3"];
        for arg in invalid_args {
            let parsed_args = IntArgs::parse(&arg.to_owned());
            assert!(parsed_args.is_none());
        }
    }
}