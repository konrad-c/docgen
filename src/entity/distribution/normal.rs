use rand_distr::Distribution;

pub struct Normal;

impl Normal {
    const DEFAULT_ROUNDING: i8 = 6;

    pub fn generate(mean: f64, stddev: f64) -> f64 {
        let dist = rand_distr::Normal::new(mean, stddev).unwrap();
        let val = dist.sample(&mut rand::thread_rng());
        math::round::floor(val, Normal::DEFAULT_ROUNDING)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn int_args_valid() {
//         let parsed_args = NormalArgs::parse(&"1,2".to_owned()).unwrap();
//         assert_eq!(parsed_args.mean, 1.0);
//         assert_eq!(parsed_args.stddev, 2.0);
//     }

//     #[test]
//     fn parse_invalid_args_returns_none() {
//         let invalid_args = vec!["", "1", "2", "string", "1,2,3"];
//         for arg in invalid_args {
//             let parsed_args = NormalArgs::parse(&arg.to_owned());
//             assert!(parsed_args.is_none());
//         }
//     }
// }