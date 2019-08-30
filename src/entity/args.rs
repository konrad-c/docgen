pub trait Args {
    fn default() -> Self;
    fn help() -> &'static str;
    fn parse(args: &String) ->Option<Self> where Self: Sized;
}