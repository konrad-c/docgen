pub mod collection;

mod location;
mod primitive;
mod name;
mod util;
mod phone;
mod distribution;

use super::types::{PlaceholderType, NameType, LocationType, PhoneType, DistributionType, PlaceholderArgs};
use super::parser::Placeholder;
use name::Name;
use location::Location;
use primitive::float::Float;
use primitive::guid::Guid;
use primitive::int::Int;
use primitive::set::Set;
use phone::Phone;
use distribution::normal::Normal;

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

    fn placeholder_parser(&mut self, placeholder: &Placeholder) -> String {
        let dtype: PlaceholderType = placeholder.data_type.clone();
        let argtype: Option<PlaceholderArgs> = placeholder.data_args.clone();

        match (dtype, argtype) {
            // Name
            (PlaceholderType::Name(NameType::First), _) => self.name.first(),
            (PlaceholderType::Name(NameType::Last), _) => self.name.last(),
            (PlaceholderType::Name(NameType::Full), _) => self.name.full(),

            // Phone
            (PlaceholderType::Phone(PhoneType::Any), _) => self.phone.phone(),
            (PlaceholderType::Phone(PhoneType::Mobile), _) => self.phone.mobile(),
            (PlaceholderType::Phone(PhoneType::Landline), _) => self.phone.landline(),

            // Location
            (PlaceholderType::Location(LocationType::Place), _) => self.location.place(),
            (PlaceholderType::Location(LocationType::Street), _) => self.location.street(),
            (PlaceholderType::Location(LocationType::Address), _) => self.location.address(),

            // Distribution
            (PlaceholderType::Distribution(DistributionType::Normal), Some(PlaceholderArgs::Normal { mean, stddev })) => Normal::generate(mean, stddev).to_string(),
            
            // Primitives 
            (PlaceholderType::Guid, _) => Guid::generate(),
            (PlaceholderType::Float, Some(PlaceholderArgs::Float { min, max })) => Float::generate(min, max).to_string(),
            (PlaceholderType::Int, Some(PlaceholderArgs::Int { min, max }))  => Int::generate(min, max).to_string(),
            (PlaceholderType::Set, Some(PlaceholderArgs::Set { options })) => Set::generate(options),
            _ => String::new()
        }
    }
}