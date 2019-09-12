pub struct Int;

impl Int {
    pub fn generate(min: i64, max: i64) -> i64 {
        let range: i64 = max - min;
        let rand_in_range: f64 = (range as f64) * rand::random::<f64>();
        return min + rand_in_range as i64;
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn int_args_valid() {
//         let parsed_args = IntArgs::parse(&"1,2".to_owned()).unwrap();
//         assert_eq!(parsed_args.min, 1);
//         assert_eq!(parsed_args.max, 2);
//     }

//     #[test]
//     fn parse_invalid_args_returns_none() {
//         let invalid_args = vec!["", "1", "2", "string", "1,2,3"];
//         for arg in invalid_args {
//             let parsed_args = IntArgs::parse(&arg.to_owned());
//             assert!(parsed_args.is_none());
//         }
//     }
// }