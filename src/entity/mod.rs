pub mod collection;

mod location;
mod primitive;
mod name;
mod util;
mod phone;
mod distribution;
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
use distribution::normal::{Normal, NormalArgs};

use args::Args;


#[derive(Debug, Clone)]
pub struct Entity {
    name: Name,
    location: Location,
    float: Float,
    guid: Guid,
    int: Int,
    set: Set,
    phone: Phone,
    normal: Normal
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
            phone: Phone::new(),
            normal: Normal::new()
        }
    }

    fn placeholder_with_args<T : Args>(&self, placeholder: &Placeholder, get_func: &mut impl FnMut(T) -> String) -> Result<String, PlaceholderParseError> {
        placeholder.args.clone()
            .and_then(|args:String| T::parse(&args))
            .map(|args: T| get_func(args).to_string())
            .ok_or(PlaceholderParseError::invalid_arg(&placeholder, T::help()))
    }

    pub fn value_of(&mut self, placeholder: Placeholder) -> Result<String, PlaceholderParseError> {
        println!("{}, {:?}", placeholder.data_type, placeholder.args);
        match placeholder.data_type.as_str() {
            name_datatype if name_datatype.starts_with("name") => self.name_placeholder(&placeholder),
            phone_datatype if phone_datatype.starts_with("phone") => self.phone_placeholder(&placeholder),
            location_datatype if location_datatype.starts_with("location") => self.location_placeholder(&placeholder),
            dist_datatype if dist_datatype.starts_with("dist") => self.dist_placeholder(&placeholder),
            "float" => self.placeholder_with_args(&placeholder, &mut |args: FloatArgs| self.float.get(args).to_string()),
            "int"  => self.placeholder_with_args(&placeholder, &mut |args: IntArgs| self.int.get(args).to_string()),
            "set" => self.placeholder_with_args(&placeholder, &mut |args: SetArgs| self.set.get(args)),
            "guid" => Ok(self.guid.get()),
            _ => Err(PlaceholderParseError::invalid_placeholder(placeholder.to_string().as_str()))
        }
    }

    fn dist_placeholder(&mut self, placeholder: &Placeholder) -> Result<String, PlaceholderParseError> {
        match placeholder.data_type.as_str() {
            "dist::normal" => self.placeholder_with_args(&placeholder, &mut |args: NormalArgs| self.normal.get(args).to_string()),
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