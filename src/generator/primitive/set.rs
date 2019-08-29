use super::util;
use super::args::Args;
use lazycell::LazyCell;

#[derive(Debug,Clone)]
pub struct Set(LazyCell<String>);

impl Set {
    pub fn new() -> Set {
        Set( LazyCell::new() )
    }

    pub fn get<'t>(&self, args: SetArgs) -> String {
        self.0.borrow_with(|| Set::generate(&args.options)).to_owned()
    }

    fn generate<'t>(set: &Vec<String>) -> String {
        let index: usize = util::rand_index(set.len());
        return set[index].to_string();
    }
}

#[derive(Debug)]
pub struct SetArgs {
    pub options: Vec<String>
}

impl Args for SetArgs {
    fn help() -> String {
        format!("")
    }

    fn default() -> SetArgs {
        SetArgs { options: vec!["A".to_owned(), "B".to_owned(), "C".to_owned()] }
    }

    fn parse(args: &String) -> Option<SetArgs> {
        let arg_vec: Vec<String> = args.split(",")
            .into_iter()
            .map(|s: &str| s.to_owned())
            .collect();
        if arg_vec.is_empty(){
            None
        } else {
            Some(SetArgs { options: arg_vec })
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
        println!("{:?}", parsed_args);
        assert!(parsed_args.is_none());
    }
}