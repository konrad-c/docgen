pub fn _string(length: usize) -> String {
    use rand::{Rng, thread_rng};
    use rand::distributions::Alphanumeric;
    
    let mut rng = thread_rng();
    return std::iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(length)
            .collect();
}

pub fn float(rounding: Option<i8>) -> f64 {
    let number: f64 = rand::random::<f64>();
    match rounding {
        Some(decimal_places) => math::round::floor(number, decimal_places),
        None => number
    }
}

pub fn int(min: i64, max: i64) -> i64 {
    let range: i64 = max - min;
    let rand_in_range: f64 = (range as f64) * rand::random::<f64>();
    return min + rand_in_range as i64;
}