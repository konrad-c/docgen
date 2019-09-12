#[macro_use]
extern crate lazy_static;

pub mod parser;
pub mod types;
mod entity;

use parser::Placeholder;
use parser::error::PlaceholderParseError;
use entity::collection::EntityCollection;
use entity::Entity;

use clap::{App, Arg, ArgMatches};
use regex::{Regex, Captures, Match};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches: ArgMatches = App::new("Templated data generator tool")
        .version("0.3.0")
        .author("Konrad <ko.cybulski@gmail.com>")
        .about("A tool for generating randomised documents in any form provided a template.
Provide a template string or file containing typed placeholders for which this tool will generate data, and create a document according to the tempalte with those placeholders replaced with randomised data.

Example templates:
    'Hi, my name is ${first_name} ${last_name}'
    'Hi, my name is ${full_name}'
    '{\"id\": \"${guid}\", \"phone\": \"${phone}\"}'

Supported data types:
    Referencing types and their subtypes is done with the syntax: 'type::subtype:arg1,arg2' e.g. 'name::first'
    Complex types:
    - name
        - first
        - last
        - full
    - location
        - address
        - place
        - street
    - phone
        - mobile
        - landline
    - dist (random distributions)
        - normal:MEAN,STANDARD_DEVIATION (normally distributed random variable with mean and standard deviation)

    Primitive types:
    - int:MIN,MAX (integer between MIN and MAX values)
    - float:MIN,MAX (float value between MIN and MAX values)
    - set:A,B,C,D (randomly selected element of the provided set e.g. B)
    - guid
        ")
        .arg(Arg::with_name("template")
            .help("The template string to populate with generated data")
            .short("t")
            .long("template")
            .takes_value(true)
            .required_unless("template-file"))
        .arg(Arg::with_name("template-file")
            .help("Path to file with template to populate with generated data")
            .short("f")
            .long("template-file")
            .takes_value(true)
            .required_unless("template"))
        .arg(Arg::with_name("number")
            .help("Number of populated documents to generate according to the template")
            .short("n")
            .takes_value(true)
            .default_value("1"))
        .get_matches();
    
    let template: String = matches.value_of("template-file")
        .and_then(|filename| std::fs::read_to_string(filename).ok())
        .or({
            matches.value_of("template").map(|t: &str| t.to_owned())
        })
        .expect("No template supplied");
    
    let errors: Vec<PlaceholderParseError> = validate_template(&template);
    if errors.len() > 0 {
        for parse_error in errors {
            println!("Validation error for placeholder '{}'. Reason: {}", parse_error.placeholder, parse_error.reason);
        }
        return Ok(());
    }

    let repetitions: u64 = matches.value_of("number")
        .unwrap_or("1")
        .parse::<u64>()
        .unwrap_or(1);
    
    for _ in 0..repetitions {
        let generated_doc: String = populate_template(&template);
        println!("{}", &generated_doc);
    }

    Ok(())
}

lazy_static! {
    static ref PLACEHOLDER_REGEX: Regex = Regex::new(r"\$\{(?:<(?P<entity_id>[a-zA-Z0-9]+)>)?(?P<placeholder>[^\}]*)\}").unwrap();
}

fn validate_template(template: &str) -> Vec<PlaceholderParseError> {
    let errors: &mut Vec<PlaceholderParseError> = &mut Vec::new();
    let x = Regex::new(r"\$\{(?:<(?P<entity_id>[a-zA-Z0-9]+)>)?(?P<placeholder>[^\}]*)\}").unwrap();
    for captures in x.captures_iter(template) {
        let placeholder_str: &str = captures.name("placeholder").unwrap().as_str();
        // Validate placeholder can be parsed to a valid type 
        if let Some(err) = Placeholder::validate(placeholder_str) {
            errors.push(err);
            continue;
        }

        let placeholder: Placeholder = Placeholder::parse(placeholder_str);
        // Validate placeholder arguments match placeholder type:
        let data_option: Option<String> = Entity::validate(&placeholder);
        if let None = data_option {
            errors.push(PlaceholderParseError { placeholder: placeholder.to_string(), reason: "Invalid arguments for placeholder type".to_owned()});
            continue;
        }
    }
    errors.clone()
}

fn populate_template(template: &str) -> String {
    let entity_collection: &mut EntityCollection = &mut EntityCollection { data: &mut HashMap::new() };

    let populated_template = PLACEHOLDER_REGEX.replace_all(template, |captures: &Captures| {
        let entity_ref: Option<&mut Entity> = captures.name("entity_id")
            .map(|id: Match| id.as_str().to_owned())
            .map(|id: String| entity_collection.get(id));

        let placeholder_str: &str = captures.name("placeholder").unwrap().as_str();
        let placeholder: Placeholder = Placeholder::parse(placeholder_str);
        match entity_ref {
            Some(entity) => entity.value_of(&placeholder),
            None => Entity::new().value_of(&placeholder)
        }
    });
    format!("{}", populated_template)
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Match;

    #[test]
    fn placeholder_regex_with_entity_id () {
        let caps: Captures = PLACEHOLDER_REGEX.captures("${<id>test}").unwrap();
        let entity_id: &str = caps.name("entity_id").unwrap().as_str();
        let placeholder: &str = caps.name("placeholder").unwrap().as_str();
        assert_eq!("id", entity_id);
        assert_eq!("test", placeholder);
    }

    #[test]
    fn placeholder_regex_without_entity_id () {
        let caps: Captures = PLACEHOLDER_REGEX.captures("${test}").unwrap();
        let entity_id: Option<Match> = caps.name("entity_id");
        let placeholder: &str = caps.name("placeholder").unwrap().as_str();
        assert_eq!(true, entity_id.is_none());
        assert_eq!("test", placeholder);
    }
}