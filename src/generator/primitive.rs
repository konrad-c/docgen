use super::util;
use uuid::Uuid;

pub struct Float;
impl Float {
    const DEFAULT_ROUNDING: i8 = 6;

    pub fn generate(min: f64, max: f64) -> f64 {
        let range: f64 = max - min;
        let rand_in_range: f64 = range * rand::random::<f64>();
        math::round::floor(rand_in_range, Float::DEFAULT_ROUNDING)
    }
}

pub struct Guid;
impl Guid {
    pub fn generate() -> String {
        Uuid::new_v4().to_hyphenated().to_string()
    }
}

pub struct Int;
impl Int {
    pub fn generate(min: i64, max: i64) -> i64 {
        let range: i64 = max - min;
        let rand_in_range: f64 = (range as f64) * rand::random::<f64>();
        return min + rand_in_range as i64;
    }
}

pub struct Set;
impl Set {
    pub fn generate<'t>(options: Vec<String>) -> String {
        let set: &Vec<String> = &options;
        let index: usize = util::rand_index(set.len());
        return set[index].to_string();
    }
}