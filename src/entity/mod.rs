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
use std::collections::HashMap;


#[derive(Debug, Clone)]
pub struct Entity {
    name: Name,
    location: Location,
    phone: Phone,
    data: HashMap<String, Option<String>>
}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            name: Name::new(),
            location: Location::new(),
            phone: Phone::new(),
            data: HashMap::new()
        }
    }

    fn placeholder_with_args<T : Args>(placeholder: &Placeholder, get_func: fn(T) -> String) -> Option<String> {
        placeholder.args.clone()
            .and_then(|args:String| T::parse(&args))
            .map(|args: T| get_func(args).to_string())
    }

    pub fn value_of(&mut self, placeholder: Placeholder) -> Option<String> {
        self.get_complex(&placeholder)
            .or_else(|| self.get_data(&placeholder))
    }

    fn get_complex(&mut self, placeholder: &Placeholder) -> Option<String> {
        match placeholder.original_type.as_str() {
            name_datatype if name_datatype.starts_with("name") => self.name_placeholder(&placeholder),
            phone_datatype if phone_datatype.starts_with("phone") => self.phone_placeholder(&placeholder),
            location_datatype if location_datatype.starts_with("location") => self.location_placeholder(&placeholder),
            _ => None
        }
    }

    fn get_data(&mut self, placeholder: &Placeholder) -> Option<String> {
        let primitive_parser = || match placeholder.original_type.as_str() {
            "guid" => Some(Guid::generate()),
            "dist::normal" => Entity::placeholder_with_args(&placeholder, |args: NormalArgs| Normal::generate(args).to_string()),
            "float" => Entity::placeholder_with_args(&placeholder, |args: FloatArgs| Float::generate(args).to_string()),
            "int"  => Entity::placeholder_with_args(&placeholder, |args: IntArgs| Int::generate(args).to_string()),
            "set" => Entity::placeholder_with_args(&placeholder, |args: SetArgs| Set::generate(args)),
            _ => None
        };
        let placeholder_string = placeholder.to_string();
        self.data.entry(placeholder_string).or_insert_with(primitive_parser).clone()
    }

    fn name_placeholder(&mut self, placeholder: &Placeholder) -> Option<String> {
        match placeholder.original_type.as_str() {
            "name::first" => Some(self.name.first()),
            "name::last" => Some(self.name.last()),
            "name::full" => Some(self.name.full()),
            _ => None
        }
    }

    fn phone_placeholder(&mut self, placeholder: &Placeholder) -> Option<String> {
        match placeholder.original_type.as_str() {
            "phone" => Some(self.phone.phone()),
            "phone::mobile" => Some(self.phone.mobile()),
            "phone::landline" => Some(self.phone.landline()),
            _ => None
        }
    }

    fn location_placeholder(&mut self, placeholder: &Placeholder) -> Option<String> {
        match placeholder.original_type.as_str() {
            "location::place" => Some(self.location.place()),
            "location::street" => Some(self.location.street()),
            "location::address" => Some(self.location.address()),
            _ => None
        }
    }
}