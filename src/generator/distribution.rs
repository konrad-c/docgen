use rand_distr::Distribution;

pub struct Normal;
impl Normal {
    const DEFAULT_ROUNDING: i8 = 6;

    pub fn generate(mean: f64, stddev: f64) -> f64 {
        let dist = rand_distr::Normal::new(mean, stddev).unwrap();
        let val = dist.sample(&mut rand::thread_rng());
        math::round::floor(val, Normal::DEFAULT_ROUNDING)
    }
}