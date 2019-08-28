// use super::placeholder::Placeholder;
use super::generator;
use super::Placeholder;
use std::collections::HashMap;

pub struct PlaceholderCollection<'t> {
    pub data: &'t mut HashMap<(String, Placeholder), String>
}

impl<'t> PlaceholderCollection<'t> {
    pub fn get(&mut self, entity_id: Option<String>, placeholder: Placeholder) -> String {
        if let Some(id) = entity_id {
            let key: (String, Placeholder) = (id, placeholder);
            if !self.data.contains_key(&key) {
                self.data.insert(key.clone(), generator::synth(placeholder));
            }
            return self.data[&key].to_owned();
        } else {
            return generator::synth(placeholder);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unidentified_will_generate_new_on_each_get(){
        let mut collection: PlaceholderCollection = PlaceholderCollection { data: &mut HashMap::new() };
        for _ in 0..20 {
            let data_first: String = collection.get(None, Placeholder::FirstName);
            let data_second: String = collection.get(None, Placeholder::FirstName);
            assert_ne!(data_first, data_second);
        }
    }

    #[test]
    fn identified_will_retrieve_already_generated_data(){
        let entity_id: Option<String> = Some(String::from("id"));
        let mut collection: PlaceholderCollection = PlaceholderCollection { data: &mut HashMap::new() };
        for _ in 0..20 {
            let data_first: String = collection.get(entity_id.clone(), Placeholder::FirstName);
            let data_second: String = collection.get(entity_id.clone(), Placeholder::FirstName);
            assert_eq!(data_first, data_second);
        }
    }
}