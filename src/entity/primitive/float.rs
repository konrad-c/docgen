pub struct Float;

impl Float {
    const DEFAULT_ROUNDING: i8 = 6;

    pub fn generate(min: f64, max: f64) -> f64 {
        let range: f64 = max - min;
        let rand_in_range: f64 = range * rand::random::<f64>();
        math::round::floor(rand_in_range, Float::DEFAULT_ROUNDING)
    }
}

// #[cfg(test)]
// mod float_args_tests {
//     use super::*;

//     #[test]
//     fn parse_valid_args() {
//         let parsed_args = FloatArgs::parse(&"1,2".to_owned()).unwrap();
//         assert_eq!(parsed_args.min, 1f64);
//         assert_eq!(parsed_args.max, 2f64);
//     }

//     #[test]
//     fn parse_invalid_args_returns_default() {
//         let invalid_args = vec!["", "string", "1,2,3"];
//         for arg in invalid_args {
//             let parsed_args = FloatArgs::parse(&arg.to_owned());
//             assert!(parsed_args.is_none());
//         }
//     }
// }