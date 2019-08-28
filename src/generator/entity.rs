use super::Placeholder;
use super::name::Name;
use super::location::Location;
use super::primitive::Primitive;
use super::phone::Phone;

#[derive(Debug, Clone)]
pub struct Entity {
    pub name: Name,
    pub location: Location,
    pub primitive: Primitive,
    pub phone: Phone
}

impl Entity {
    pub fn new() -> Entity {
        Entity {
            name: Name::new(),
            location: Location::new(),
            primitive: Primitive::new(),
            phone: Phone::new()
        }
    }

    pub fn value_of(&mut self, placeholder: Placeholder) -> String {
        println!("{}, {:?}", placeholder.data_type, placeholder.args);
        match placeholder.data_type.as_str() {
            "name::first" => self.name.first(),
            "name::last" => self.name.last(),
            "name::full" => self.name.full(),
            "phone" => self.phone.phone(),
            "phone::mobile" => self.phone.mobile(),
            "phone::landline" => self.phone.landline(),
            "place" => self.location.place(),
            "address" => self.location.address(),
            "float" => self.primitive.float(&placeholder.args).to_string(),
            "int"  => self.primitive.int(&placeholder.args).to_string(),
            "guid" => self.primitive.guid(),
            "set" => self.primitive.set(&placeholder.args),
            _ => String::new()
        }
    }

    // fn phone(placeholder: Placeholder) -> String {
    //     match (placeholder.data_type, placeholder.args) {

    //     }
    // }
}