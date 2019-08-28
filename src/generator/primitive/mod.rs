mod int_args;
mod float_args;
mod set_args;

use super::util;
use int_args::IntArgs;
use float_args::FloatArgs;
use set_args::SetArgs;

use lazycell::LazyCell;
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

    pub fn float(&self, args_string: &Option<String>) -> f64 {
        *self.float.borrow_with(|| {
            let generator_args: FloatArgs = match args_string {
                Some(args) => FloatArgs::parse(args),
                None => FloatArgs::new()
            };
            PrimitiveGenerator::float(generator_args.rounding)
        })
    }

    pub fn int<'t>(&self, args_string: &Option<String>) -> i64 {
        *self.int.borrow_with(|| {
            let generator_args: IntArgs = match args_string {
                Some(args) => IntArgs::parse(args),
                None => IntArgs::new()
            };
            PrimitiveGenerator::int(generator_args.min, generator_args.max)
        })
    }

    pub fn set(&self, args_string: &Option<String>) -> String {
        self.set.borrow_with(|| {
            let generator_args: SetArgs = match args_string {
                Some(args) => SetArgs::parse(args),
                None => SetArgs::new()
            };
            PrimitiveGenerator::from_set(&generator_args.options)
        }).to_owned()
    }
}

pub struct PrimitiveGenerator;
impl PrimitiveGenerator {
    pub fn float(rounding: i8) -> f64 {
        let number: f64 = rand::random::<f64>();
        math::round::floor(number, rounding)
    }

    pub fn guid() -> String {
        Uuid::new_v4().to_hyphenated().to_string()
    }

    pub fn int(min: i64, max: i64) -> i64 {
        let range: i64 = max - min;
        let rand_in_range: f64 = (range as f64) * rand::random::<f64>();
        return min + rand_in_range as i64;
    }

    pub fn from_set<'t>(set: &Vec<&'t str>) -> String {
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