use clap::{App, Arg};

fn main() {
    let _matches = App::new("Templated data generator tool")
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
            .long("template"))
        .arg(Arg::with_name("template file")
            .help("The template string to populate with generated data")
            .long("template-file"))
        .get_matches();

    test_generative_fn("float", 2, || { rand::random::<f64>().to_string() });
    test_generative_fn("int", 2, || { rand::random::<u64>().to_string() });
    test_generative_fn("string", 2, || { string_gen::generate(10) });
    test_generative_fn("firstName", 5, || { String::from(generate_first_name()) });
    test_generative_fn("lastName", 5, || { String::from(generate_last_name()) });
    test_generative_fn("middleName", 5, || { String::from(generate_middle_name()) });
    test_generative_fn("place", 5, || { String::from(generate_place()) });
}

fn test_generative_fn<F>(fn_name: &str, number:usize, f: F) where F: Fn() -> String {
    for _ in 1..number {
        let val: String = f();
        println!("{}: {:?}", fn_name, val);
    }
}

mod string_gen {
    pub fn generate(length: usize) -> String {
        use rand::{Rng, thread_rng};
        use rand::distributions::Alphanumeric;
        
        let mut rng = thread_rng();
        return std::iter::repeat(())
                .map(|()| rng.sample(Alphanumeric))
                .take(length)
                .collect();
    }
}

fn rand_index(length: usize) -> usize {
    let index_approx: f64 = (length as f64) * rand::random::<f64>();
    return math::round::floor(index_approx, 0) as usize;
}

mod first_names;
use first_names::FIRST_NAMES;
fn generate_first_name() -> &'static str {
    let index: usize = rand_index(FIRST_NAMES.len());
    return FIRST_NAMES[index];
}

mod last_names;
use last_names::LAST_NAMES;
fn generate_last_name() -> &'static str {
    let index: usize = rand_index(LAST_NAMES.len());
    return LAST_NAMES[index];
}

mod middle_names;
use middle_names::MIDDLE_NAMES;
fn generate_middle_name() -> &'static str {
    let index: usize = rand_index(MIDDLE_NAMES.len());
    return MIDDLE_NAMES[index];
}

mod places;
use places::PLACES;
fn generate_place() -> &'static str {
    let index: usize = rand_index(PLACES.len());
    return PLACES[index];
}
// mod Generator {

//     fn integer(min: u64, max: u64) -> u64 {
//     }

//     fn randInt(randSource: Source, min: u64, max: u64) -> u64 {
//         let float: f64 = randSource.read_f64();
//         let result = min + (max - min) * float; 
//     }
// }