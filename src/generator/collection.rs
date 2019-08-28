use super::entity::Entity;
use std::collections::HashMap;

pub struct EntityCollection<'t> {
    pub data: &'t mut HashMap<String, Entity>
}

impl<'t> EntityCollection<'t> {
    pub fn get(&mut self, id: String) -> &mut Entity {
        if !self.data.contains_key(&id) {
            self.data.insert(id.clone(), Entity::new());
        }
        return self.data.get_mut(&id).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identified_will_retrieve_already_generated_data(){
        let mut collection: EntityCollection = EntityCollection { data: &mut HashMap::new() };
        for _ in 0..20 {
            let first_id: String = String::from("id");
            let second_id: String = first_id.clone();
            let data_first: String = collection.get(first_id).name.first();
            let data_second: String = collection.get(second_id).name.first();
            assert_eq!(data_first, data_second);
        }
    }
}