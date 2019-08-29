use lazycell::LazyCell;
use super::args::Args;

#[derive(Debug,Clone)]
pub struct Float(LazyCell<f64>);

impl Float {
    pub fn new() -> Float {
        Float( LazyCell::new() )
    }

    pub fn get<'t>(&self, args: FloatArgs) -> f64 {
        *self.0.borrow_with(|| Float::generate(args.rounding))
    }

    fn generate(rounding: i8) -> f64 {
        let number: f64 = rand::random::<f64>();
        math::round::floor(number, rounding)
    }
}

#[derive(Debug)]
pub struct FloatArgs {
    pub rounding: i8
}

impl Args for FloatArgs {
    fn help() -> String {
        format!("")
    }

    fn default() -> FloatArgs {
        FloatArgs { rounding: 5 }
    }

    fn parse<'t>(args: &String) -> Option<FloatArgs> {
        let arg_vec: Vec<&str> = args.split(",")
            .into_iter()
            .collect();
        if let [rounding] = &arg_vec[..] {
            return rounding.parse::<i8>().ok()
                .map(|decimal_places: i8| FloatArgs { rounding: decimal_places });
        }
        return None;
    }
}

#[cfg(test)]
mod int_args_tests {
    use super::*;

    #[test]
    fn parse_valid_args() {
        let parsed_args = FloatArgs::parse(&"1".to_owned()).unwrap();
        assert_eq!(parsed_args.rounding, 1);
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