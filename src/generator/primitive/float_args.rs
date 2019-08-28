#[derive(Debug)]
pub struct FloatArgs {
    pub rounding: i8
}

impl FloatArgs {
    const DEFAULT_ROUNDING: i8 = 5;

    pub fn new() -> FloatArgs {
        FloatArgs { rounding: FloatArgs::DEFAULT_ROUNDING }
    }

    pub fn parse<'t>(args: &'t str) -> FloatArgs {
        let arg_vec: Vec<&str> = args.split(",")
            .into_iter()
            .collect();
        match &arg_vec[..] {
            [rounding] => match rounding.parse::<i8>() {
                Ok(decimal_places) => FloatArgs { rounding: decimal_places },
                _ => FloatArgs::new()
            },
            _ => FloatArgs::new()
        }
    }
}

#[cfg(test)]
mod int_args_tests {
    use super::*;

    #[test]
    fn parse_valid_args() {
        let parsed_args = FloatArgs::parse("1");
        assert_eq!(parsed_args.rounding, 1);
    }

    #[test]
    fn parse_invalid_args_returns_default() {
        let invalid_args = vec!["", "string", "1,2,3"];
        for arg in invalid_args {
            let parsed_args = FloatArgs::parse(arg);
            assert_eq!(parsed_args.rounding, FloatArgs::DEFAULT_ROUNDING);
        }
    }
}