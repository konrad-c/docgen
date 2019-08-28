use lazycell::LazyCell;
use super::util;
use uuid::Uuid;

#[derive(Debug,Clone)]
pub struct Primitive {
    float: LazyCell<f64>,
    int: LazyCell<i64>,
    guid: LazyCell<String>,
    set: LazyCell<String>
}

impl Primitive {
    pub fn new() -> Primitive {
        Primitive {
            float: LazyCell::new(),
            int: LazyCell::new(),
            guid: LazyCell::new(),
            set: LazyCell::new()
        }
    }

    pub fn guid(&self) -> String {
        self.guid.borrow_with(PrimitiveGenerator::guid).to_owned()
    }

    pub fn float(&self) -> f64 {
        *self.float.borrow_with(|| PrimitiveGenerator::float(None))
    }

    pub fn int(&self) -> i64 {
        *self.int.borrow_with(|| PrimitiveGenerator::int(0, 10))
    }

    pub fn set(&self, options: &Vec<String>) -> String {
        self.set.borrow_with(|| PrimitiveGenerator::from_set(options)).to_owned()
    }
}

pub struct PrimitiveGenerator;
impl PrimitiveGenerator {
    pub fn float(rounding: Option<i8>) -> f64 {
        let number: f64 = rand::random::<f64>();
        match rounding {
            Some(decimal_places) => math::round::floor(number, decimal_places),
            None => number
        }
    }

    pub fn guid() -> String {
        Uuid::new_v4().to_hyphenated().to_string()
    }

    pub fn int(min: i64, max: i64) -> i64 {
        let range: i64 = max - min;
        let rand_in_range: f64 = (range as f64) * rand::random::<f64>();
        return min + rand_in_range as i64;
    }

    pub fn from_set(set: &Vec<String>) -> String {
        let index: usize = util::rand_index(set.len());
        return set[index].to_string();
    }
    
    fn _string(length: usize) -> String {
        use rand::{Rng, thread_rng};
        use rand::distributions::Alphanumeric;
        
        let mut rng = thread_rng();
        return std::iter::repeat(())
                .map(|()| rng.sample(Alphanumeric))
                .take(length)
                .collect();
    }
}