pub fn rand_index(length: usize) -> usize {
    let index_approx: f64 = (length as f64) * rand::random::<f64>();
    return math::round::floor(index_approx, 0) as usize;
}