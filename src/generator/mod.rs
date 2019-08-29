pub mod collection;

mod location;
mod primitive;
mod name;
mod util;
mod phone;
mod args;

use super::placeholder::{Placeholder};
use name::Name;
use location::Location;
use primitive::float::{Float,FloatArgs};
use primitive::guid::Guid;
use primitive::int::{Int,IntArgs};
use primitive::set::{Set,SetArgs};
use phone::Phone;

use args::Args;


#[derive(Debug, Clone)]
pub struct Entity {
    pub name: Name,
    pub location: Location,
    pub float: Float,
    pub guid: Guid,
    pub int: Int,
    pub set: Set,
    pub phone: Phone
}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            name: Name::new(),
            location: Location::new(),
            float: Float::new(),
            guid: Guid::new(),
            int: Int::new(),
            set: Set::new(),
            phone: Phone::new()
        }
    }

    fn get_fn_and_parser(placeholder: Placeholder) -> Option<impl Args> {
        if let Some(args) = &placeholder.args {
            return match placeholder.data_type.as_str() {
                "int" => IntArgs::parse(&args),
                _ => None
            };
        }
        None
    }

    pub fn value_of(&mut self, placeholder: Placeholder) -> Option<String> {
        println!("{}, {:?}", placeholder.data_type, placeholder.args);
        match placeholder.data_type.as_str() {
            "name::first" => Some(self.name.first()),
            "name::last" => Some(self.name.last()),
            "name::full" => Some(self.name.full()),
            "phone" => Some(self.phone.phone()),
            "phone::mobile" => Some(self.phone.mobile()),
            "phone::landline" => Some(self.phone.landline()),
            "place" => Some(self.location.place()),
            "address" => Some(self.location.address()),
            "float" => placeholder.args
                .and_then(|args:String| FloatArgs::parse(&args))
                .map(|args: FloatArgs| self.float.get(args).to_string()),
            "int"  => placeholder.args
                .and_then(|args:String| IntArgs::parse(&args))
                .map(|args: IntArgs| self.int.get(args).to_string()),
            "guid" => Some(self.guid.get()),
            "set" => placeholder.args
                .and_then(|args:String| SetArgs::parse(&args))
                .map(|args: SetArgs| self.set.get(args)),
            _ => None
        }
    }

    // fn phone(placeholder: Placeholder) -> String {
    //     match (placeholder.data_type, placeholder.args) {

    //     }
    // }
}