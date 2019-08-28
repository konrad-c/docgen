#[derive(Debug)]
pub struct SetArgs<'t> {
    pub options: Vec<&'t str>
}

impl<'t> SetArgs<'t> {
    pub fn new() -> SetArgs<'t> {
        SetArgs { options: vec!["A", "B", "C"] }
    }

    pub fn parse(args: &'t str) -> SetArgs {
        let arg_vec: Vec<&str> = args.split(",")
            .into_iter()
            .collect();
        SetArgs { options: arg_vec }
    }
}

#[cfg(test)]
mod int_args_tests {
    use super::*;

    #[test]
    fn parse_valid_args() {
        let parsed_args = SetArgs::parse("1,2");
        assert_eq!(parsed_args.options, vec!["1","2"]);
    }
}