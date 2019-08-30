pub mod collection;

mod location;
mod primitive;
mod name;
mod util;
mod phone;
mod args;

use super::placeholder::Placeholder;
use super::placeholder::error::PlaceholderParseError;
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

    pub fn value_of(&mut self, placeholder: Placeholder) -> Result<String, PlaceholderParseError> {
        println!("{}, {:?}", placeholder.data_type, placeholder.args);
        match placeholder.data_type.as_str() {
            name_datatype if name_datatype.starts_with("name") => self.name_placeholder(&placeholder),
            phone_datatype if phone_datatype.starts_with("phone") => self.phone_placeholder(&placeholder),
            location_datatype if location_datatype.starts_with("location") => self.location_placeholder(&placeholder),
            "float" => placeholder.args.clone()
                .and_then(|args:String| FloatArgs::parse(&args))
                .map(|args: FloatArgs| self.float.get(args).to_string())
                .ok_or(PlaceholderParseError::invalid_arg(&placeholder, FloatArgs::help())),
            "int"  => placeholder.args.clone()
                .and_then(|args:String| IntArgs::parse(&args))
                .map(|args: IntArgs| self.int.get(args).to_string())
                .ok_or(PlaceholderParseError::invalid_arg(&placeholder, IntArgs::help())),
            "guid" => Ok(self.guid.get()),
            "set" => placeholder.args.clone()
                .and_then(|args:String| SetArgs::parse(&args))
                .map(|args: SetArgs| self.set.get(args))
                .ok_or(PlaceholderParseError::invalid_arg(&placeholder, SetArgs::help())),
            _ => Err(PlaceholderParseError::invalid_placeholder(placeholder.to_string().as_str()))
        }
    }

    fn name_placeholder(&mut self, placeholder: &Placeholder) -> Result<String, PlaceholderParseError> {
        match placeholder.data_type.as_str() {
            "name::first" => Ok(self.name.first()),
            "name::last" => Ok(self.name.last()),
            "name::full" => Ok(self.name.full()),
            _ => Err(PlaceholderParseError::invalid_placeholder(placeholder.to_string().as_str()))
        }
    }

    fn phone_placeholder(&mut self, placeholder: &Placeholder) -> Result<String, PlaceholderParseError> {
        match placeholder.data_type.as_str() {
            "phone" => Ok(self.phone.phone()),
            "phone::mobile" => Ok(self.phone.mobile()),
            "phone::landline" => Ok(self.phone.landline()),
            _ => Err(PlaceholderParseError::invalid_placeholder(placeholder.to_string().as_str()))
        }
    }

    fn location_placeholder(&mut self, placeholder: &Placeholder) -> Result<String, PlaceholderParseError> {
        match placeholder.data_type.as_str() {
            "location::place" => Ok(self.location.place()),
            "location::street" => Ok(self.location.street()),
            "location::address" => Ok(self.location.address()),
            _ => Err(PlaceholderParseError::invalid_placeholder(placeholder.to_string().as_str()))
        }
    }
}