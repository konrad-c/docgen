// use super::placeholder::Placeholder;
use super::{synth};
use super::Placeholder;
use std::collections::HashMap;

pub struct DataCollection<'t> {
    pub data: &'t mut HashMap<Placeholder, String>
}

impl<'t> DataCollection<'t> {
    pub fn get(&mut self, placeholder: Placeholder) -> String {
        if let Some(id) = placeholder.clone().id {
            if !self.data.contains_key(&placeholder) {
                self.data.insert(placeholder.clone(), synth(placeholder.clone()));
            }
            return self.data[&placeholder].to_owned();
        } else {
            return synth(placeholder);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unidentified_will_generate_new_on_each_get(){
        let mut collection: DataCollection = DataCollection { data: &mut HashMap::new() };
        for _ in 0..20 {
            let data_first: String = collection.get(Placeholder::parse("name::first").unwrap());
            let data_second: String = collection.get(Placeholder::parse("name::first").unwrap());
            assert_ne!(data_first, data_second);
        }
    }

    #[test]
    fn identified_will_retrieve_already_generated_data(){
        let entity_id: Option<String> = Some(String::from("id"));
        let mut collection: DataCollection = DataCollection { data: &mut HashMap::new() };
        for _ in 0..20 {
            let data_first: String = collection.get(Placeholder::parse("<id>name::first").unwrap());
            let data_second: String = collection.get(Placeholder::parse("<id>name::first").unwrap());
            assert_eq!(data_first, data_second);
        }
    }
}