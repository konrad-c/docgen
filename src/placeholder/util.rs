pub fn rand_index(length: usize) -> usize {
    let index_approx: f64 = (length as f64) * rand::random::<f64>();
    return math::round::floor(index_approx, 0) as usize;
}

pub fn from_set(set: &Vec<String>) -> String {
    let index: usize = rand_index(set.len());
    return set[index].to_string();
}