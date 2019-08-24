#[macro_use]
extern crate lazy_static;

mod name;
mod location;
mod primitive;
mod util;
mod placeholder;
mod generator;

use placeholder::{Placeholder, PlaceholderParseError};
use clap::{App, Arg, ArgMatches};
use regex::{Regex, Captures};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches: ArgMatches = App::new("Templated data generator tool")
        .version("0.0.1")
        .author("Konrad <ko.cybulski@gmail.com>")
        .about("A tool for generating randomised documents in any form provided a template.
Provide a template string or file containing typed placeholders for which this tool will generate data, and create a document according to the tempalte with those placeholders replaced with randomised data.

Example templates:
    'Hi, my name is ${firstName} ${lastName}'
    'Hi, my name is ${fullName}'
    '{\"id\": \"${guid}\", \"phone\": \"${phoneNumber}\"}'

Supported data types:
    - firstName
    - lastName
    - fullName
    - phoneNumber
    - address
    - guid
    - int (integer between 0 and 10)
    - int:MIN,MAX (integer between MIN and MAX values)
    - float (values default between 0 and 1)
    - float:ROUNDING (number of decimal places to round float value)
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


    // let template_file: Option<String> = matches.value_of("template-file")
    //     .and_then(|filename| std::fs::read_to_string(filename).ok());
    // let template_arg: Option<String> = matches.value_of("template")
    //     .map(|t: &str| t.to_owned());
    // let sourced_template: String = template_file.or(template_arg).unwrap();


    let repetitions: u64 = matches.value_of("number")
        .unwrap_or("1")
        .parse::<u64>()
        .unwrap_or(1);
    
    for i in 0..repetitions {
        let generated_doc: String = populate_template(&template);
        println!("Populating document #{}", i);
        println!("{}", generated_doc);
    }

    Ok(())
}

fn populate_template(template: &str) -> String {
    let placeholder_regex: Regex = Regex::new(r"\$\{(?P<placeholder>.*)\}").unwrap();
    let populated_template = placeholder_regex.replace_all(template, |captures: &Captures| {
        let matched_text: &str = captures.get(0).unwrap().as_str();
        let placeholder_str: &str = captures.name("placeholder").unwrap().as_str();

        let parsed_placeholder: Result<Placeholder, PlaceholderParseError> = placeholder::parse(placeholder_str);
        let placeholder_data: Result<String, PlaceholderParseError> = parsed_placeholder.and_then(|placeholder| generator::generate_data(placeholder));
        match placeholder_data {
            Ok(data) => {
                println!("Matched: {}, Generated data: {}", matched_text, data);
                return data;
            },
            Err(parse_error) => {
                println!("Error: '{}' failed to parse on token '{}' because: {}", matched_text, parse_error.token, parse_error.reason);
                return format!("{}", matched_text);
            }
        }
    });
    format!("{}", populated_template)
}