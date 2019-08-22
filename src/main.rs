mod name;
mod location;
mod primitive;
mod util;
use clap::{App, Arg};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Templated data generator tool")
        .version("0.0.1")
        .author("Konrad <ko.cybulski@gmail.com>")
        .about("A tool for generating randomised documents in any form provided a template.
Provide a template string or file containing typed fields for which this tool will generate data, and create a document according to the tempalte with those fields replaced with randomised data.

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
    - integer
    - float (values default between 0 and 1)
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
        .get_matches();

    let template_from_file: Option<String> = matches.value_of("template-file")
        .map(|filename| std::fs::read_to_string(filename).unwrap());
    let template_arg: Option<String> = matches.value_of("template")
        .map(|template_string| template_string.to_owned());

    let populated_template: Option<String> = template_from_file.or(template_arg)
        .map(|t| t.replace("${float}", &primitive::float(None).to_string() ))
        .map(|t| t.replace("${int}", &primitive::int(-10, 10).to_string() ))
        .map(|t| t.replace("${string}", &primitive::string(10)))
        .map(|t| t.replace("${firstName}", &name::first() ))
        .map(|t| t.replace("${lastName}", &name::last() ))
        .map(|t| t.replace("${fullName}", &name::full() ))
        .map(|t| t.replace("${place}", &location::place() ));
    
    match populated_template {
        Some(value) => println!("{}", value),
        None => println!("No template supplied")
    }

    Ok(())
}

fn test_generative_fn<F>(fn_name: &str, number:usize, f: F) where F: Fn() -> String {
    for _ in 1..number {
        let val: String = f();
        println!("{}: {:?}", fn_name, val);
    }
}