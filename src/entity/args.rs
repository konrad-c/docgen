pub trait Args {
    fn default() -> Self;
    fn help() -> String;
    fn parse(args: &String) -> Option<Self> where Self: Sized;
}