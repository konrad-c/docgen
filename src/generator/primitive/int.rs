#[derive(Debug)]
pub struct IntArgs {
    pub min: i64,
    pub max: i64
}

impl IntArgs {
    const DEFAULT_MIN: i64 = 0;
    const DEFAULT_MAX: i64 = 10;

    pub fn new() -> IntArgs {
        IntArgs { min: IntArgs::DEFAULT_MIN, max: IntArgs::DEFAULT_MAX }
    }

    pub fn parse<'t>(args: &'t str) -> IntArgs {
        let arg_vec: Vec<&str> = args.split(",")
            .into_iter()
            .collect();
        match &arg_vec[..] {
            [min,max] => match (min.parse::<i64>(), max.parse::<i64>()) {
                (Ok(min_val), Ok(max_val)) => IntArgs { min: min_val, max: max_val },
                _ => IntArgs::new()
            },
            _ => IntArgs::new()
        }
    }
}

#[cfg(test)]
mod int_args_tests {
    use super::*;

    #[test]
    fn parse_valid_args() {
        let parsed_args = IntArgs::parse("1,2");
        assert_eq!(parsed_args.min, 1);
        assert_eq!(parsed_args.max, 2);
    }

    #[test]
    fn parse_invalid_args_returns_default() {
        let invalid_args = vec!["", "1", "2", "string", "1,2,3"];
        for arg in invalid_args {
            let parsed_args = IntArgs::parse(arg);
            assert_eq!(parsed_args.min, IntArgs::DEFAULT_MIN);
            assert_eq!(parsed_args.max, IntArgs::DEFAULT_MAX);
        }
    }
}