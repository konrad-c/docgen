pub mod collection;

mod location;
mod primitive;
mod name;
mod util;
mod phone;
mod distribution;
mod args;

use super::placeholder::types::{PlaceholderType, NameType, LocationType, PhoneType, DistributionType};
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
    data: HashMap<String, String>
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

    pub fn value_of(&mut self, placeholder: &Placeholder) -> String {
        let placeholder_string = placeholder.to_string();
        if self.data.contains_key(&placeholder_string) {
            return self.data.get(&placeholder_string).unwrap().to_string();
        } else {
            let generated_data: String = self.placeholder_parser(&placeholder);
            let returned_data: String = generated_data.clone();
            self.data.insert(placeholder_string, generated_data);
            return returned_data;
        }
    }

    fn placeholder_with_args<T : Args>(placeholder: &Placeholder, get_func: fn(T) -> String) -> String {
        let args: String = placeholder.args.clone().unwrap();
        let parsed_args: Option<T> = T::parse(&args);
        get_func(parsed_args.unwrap()).to_string()
    }

    fn placeholder_parser(&mut self, placeholder: &Placeholder) -> String {
        match placeholder.data_type {
            // PlaceholderType::Name(_) => self.name_placeholder(&placeholder),
            PlaceholderType::Name(NameType::First) => self.name.first(),
            PlaceholderType::Name(NameType::Last) => self.name.last(),
            PlaceholderType::Name(NameType::Full) => self.name.full(),

            // PlaceholderType::Phone(_) => self.phone_placeholder(&placeholder),
            PlaceholderType::Phone(PhoneType::Any) => self.phone.phone(),
            PlaceholderType::Phone(PhoneType::Mobile) => self.phone.mobile(),
            PlaceholderType::Phone(PhoneType::Landline) => self.phone.landline(),

            // PlaceholderType::Location(_) => self.location_placeholder(&placeholder),
            PlaceholderType::Location(LocationType::Place) => self.location.place(),
            PlaceholderType::Location(LocationType::Street) => self.location.street(),
            PlaceholderType::Location(LocationType::Address) => self.location.address(),
            
            // Primitives 
            PlaceholderType::Guid => Guid::generate(),
            PlaceholderType::Distribution(DistributionType::Normal) => Entity::placeholder_with_args(&placeholder, |args: NormalArgs| Normal::generate(args).to_string()),
            PlaceholderType::Float => Entity::placeholder_with_args(&placeholder, |args: FloatArgs| Float::generate(args).to_string()),
            PlaceholderType::Int  => Entity::placeholder_with_args(&placeholder, |args: IntArgs| Int::generate(args).to_string()),
            PlaceholderType::Set => Entity::placeholder_with_args(&placeholder, |args: SetArgs| Set::generate(args))
        }
    }
}