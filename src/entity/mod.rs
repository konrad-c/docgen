pub mod collection;

use crate::types::{PlaceholderType, NameType, LocationType, PhoneType, DistributionType, PlaceholderArgs};
use crate::parser::Placeholder;
use crate::generator::name::Name;
use crate::generator::location::Location;
use crate::generator::primitive::{Float,Guid,Int,Set};
use crate::generator::phone::Phone;
use crate::generator::distribution::Normal;

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
            let generated_data: String = self.generate(&placeholder).unwrap();
            let returned_data: String = generated_data.clone();
            self.data.insert(placeholder_string, generated_data);
            return returned_data;
        }
    }

    pub fn validate(placeholder: &Placeholder) -> Option<String> {
        Entity::new().generate(&placeholder)
    }

    fn generate(&mut self, placeholder: &Placeholder) -> Option<String> {
        let dtype: PlaceholderType = placeholder.data_type.clone();
        let argtype: Option<PlaceholderArgs> = placeholder.data_args.clone();

        match dtype {
            PlaceholderType::Name(name_type) => self.generate_name(&name_type),
            PlaceholderType::Location(location_type) => self.generate_location(&location_type),
            PlaceholderType::Phone(phone_type) => self.generate_phone(&phone_type),
            PlaceholderType::Distribution(distribution_type) => self.generate_distribution(&distribution_type, argtype),
            PlaceholderType::Float => self.generate_float(argtype),
            PlaceholderType::Int => self.generate_int(argtype),
            PlaceholderType::Set => self.generate_set(argtype),
            PlaceholderType::Guid => Some(Guid::generate()),
        }
    }

    fn generate_name(&mut self, dtype: &NameType) -> Option<String> {
        match dtype {
            NameType::First => Some(self.name.first()),
            NameType::Last => Some(self.name.last()),
            NameType::Full => Some(self.name.full())
        }
    }

    fn generate_location(&mut self, dtype: &LocationType) -> Option<String> {
        match dtype {
            LocationType::Place => Some(self.location.place()),
            LocationType::Street => Some(self.location.street()),
            LocationType::Address => Some(self.location.address())
        }
    }

    fn generate_phone(&mut self, dtype: &PhoneType) -> Option<String> {
        match dtype {
            PhoneType::Any => Some(self.phone.phone()),
            PhoneType::Mobile => Some(self.phone.mobile()),
            PhoneType::Landline => Some(self.phone.landline())
        }
    }

    fn generate_distribution(&mut self, dtype: &DistributionType, argtype: Option<PlaceholderArgs>) -> Option<String> {
        argtype.and_then(|args: PlaceholderArgs| match (dtype, args) {
            (DistributionType::Normal, PlaceholderArgs::Normal { mean, stddev }) => Some(Normal::generate(mean, stddev).to_string()),
            _ => None
        })
    }

    fn generate_int(&mut self, args: Option<PlaceholderArgs>) -> Option<String> {
        match args {
            Some(PlaceholderArgs::Int { min, max })  => Some(Int::generate(min, max).to_string()),
            Some(PlaceholderArgs::IntRepeated { min, max, repeat })  => {
                let generated: Vec<String> = (0..repeat)
                    .map(|_: u64| Int::generate(min, max).to_string())
                    .collect();
                Some(generated.join(""))
            },
            _ => None
        }
    }

    fn generate_float(&mut self, argtype: Option<PlaceholderArgs>) -> Option<String> {
        argtype.and_then(|args: PlaceholderArgs| match args {
            PlaceholderArgs::Float { min, max } => Some(Float::generate(min, max).to_string()),
            _ => None
        })
    }

    fn generate_set(&mut self, argtype: Option<PlaceholderArgs>) -> Option<String> {
        argtype.and_then(|args: PlaceholderArgs| match args {
            PlaceholderArgs::Set { options } => Some(Set::generate(options)),
            _ => None
        })
    }
}