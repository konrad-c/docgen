// use super::placeholder::Placeholder;
use super::generator;
use super::Placeholder;
use std::collections::HashMap;

pub struct PlaceholderCollection<'t> {
    pub data: &'t mut HashMap<(String, Placeholder), String>
}

impl<'t> PlaceholderCollection<'t> {
    // pub fn new() -> &'static mut PlaceholderCollection<'t> {
    //     return &mut PlaceholderCollection { data: &mut HashMap::new() };
    // }

    pub fn get(&mut self, entity_id: Option<String>, placeholder: Placeholder) -> String {
        if let Some(id) = entity_id {
            let key: (String, Placeholder) = (id, placeholder);
            if !self.data.contains_key(&key) {
                self.data.insert(key.clone(), generator::synth(placeholder));
            }
            return self.data[&key].to_owned();//.get(&key).cloned().unwrap();
        } else {
            return generator::synth(placeholder);
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn unidentified_will_generate_new_on_each_get(){
//         let mut none_entity: Entity = Entity::new(None);
//         for _ in 0..20 {
//             let data_first: String = none_entity.value_of(Placeholder::FirstName);
//             let data_second: String = none_entity.value_of(Placeholder::FirstName);
//             assert_ne!(data_first, data_second);
//         }
//     }

//     #[test]
//     fn identified_will_retrieve_already_generated_data(){
//         let entity_id: String = String::from("id");
//         let mut entity_with_id: Entity = Entity::new(Some(entity_id));
//         for _ in 0..20 {
//             let data_first: String = entity_with_id.value_of(Placeholder::FirstName);
//             let data_second: String = entity_with_id.value_of(Placeholder::FirstName);
//             assert_eq!(data_first, data_second);
//         }
//     }
// }