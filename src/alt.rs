use super::generator;
use super::generator::collection::DataCollection;
use regex::{Regex, Captures, Match};
use std::collections::HashMap;

lazy_static! {
    static ref PLACEHOLDER_REGEX: Regex = Regex::new(r"\$\{(?:<(?P<entity_id>[a-zA-Z0-9]+)>)?(?P<placeholder>[^\}]*)\}").unwrap();
}

fn populate_template(template: &str) -> String {
    let placeholder_collection: &mut DataCollection = &mut DataCollection { data: &mut HashMap::new() };

    let populated_template = PLACEHOLDER_REGEX.replace_all(template, |captures: &Captures| {
        // let matched_text: String = captures.get(0).unwrap().as_str().to_owned(); 

        // let placeholder_str: &str = captures.name("placeholder").unwrap().as_str();
        // let placeholder_data: Result<Placeholder, PlaceholderParseError> = Placeholder::parse(placeholder_str);

        // match placeholder_data {
        //     Ok(placeholder) => placeholder_collection.get(placeholder),
        //     Err(parse_error) => {
        //         println!("Error: '{}' failed to parse on token '{}' because: {}", matched_text, parse_error.token, parse_error.reason);
        //         return format!("{}", matched_text);
        //     }
        // }
        ""
    });
    format!("{}", populated_template)
}