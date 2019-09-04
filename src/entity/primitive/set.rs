use super::util;
use super::args::Args;

pub struct Set;

impl Set {
    pub fn generate<'t>(args: SetArgs) -> String {
        let set: &Vec<String> = &args.options;
        let index: usize = util::rand_index(set.len());
        return set[index].to_string();
    }
}

#[derive(Debug)]
pub struct SetArgs {
    pub options: Vec<String>
}

impl Args for SetArgs {
    fn help() -> &'static str {
        ""
    }

    fn default() -> SetArgs {
        SetArgs { options: vec!["A".to_owned(), "B".to_owned(), "C".to_owned()] }
    }

    fn parse(args: &String) -> Option<SetArgs> {
        match args.is_empty() {
            true => None,
            false => Some(SetArgs { 
                options :args.split(",")
                    .into_iter()
                    .map(|s: &str| s.to_owned())
                    .collect()
            })
        }
    }
}

#[cfg(test)]
mod int_args_tests {
    use super::*;

    #[test]
    fn parse_valid_args() {
        let parsed_args = SetArgs::parse(&"1,2".to_owned()).unwrap();
        assert_eq!(parsed_args.options, vec!["1","2"]);
    }

    #[test]
    fn parse_empty_returns_none() {
        let parsed_args = SetArgs::parse(&String::new());
        assert!(parsed_args.is_none());
    }
}