use super::util;

pub struct Set;

impl Set {
    pub fn generate<'t>(options: Vec<String>) -> String {
        let set: &Vec<String> = &options;
        let index: usize = util::rand_index(set.len());
        return set[index].to_string();
    }
}

// #[cfg(test)]
// mod int_args_tests {
//     use super::*;

//     #[test]
//     fn parse_valid_args() {
//         let parsed_args = SetArgs::parse(&"1,2".to_owned()).unwrap();
//         assert_eq!(parsed_args.options, vec!["1","2"]);
//     }

//     #[test]
//     fn parse_empty_returns_none() {
//         let parsed_args = SetArgs::parse(&String::new());
//         assert!(parsed_args.is_none());
//     }
// }